use anyhow::{anyhow, Result};
use deciduously_com_ui as ui;
use std::path::{Path, PathBuf};
use sunfish::{include_dir, include_dir::IncludeDir};
use url::Url;

pub struct BlogPost;

#[derive(serde::Deserialize)]
pub struct BlogPostFrontMatter {
	pub title: String,
	pub date: String,
	pub tags: Option<String>,
	pub cover_image: Option<Url>,
}

impl Content for BlogPost {
	type FrontMatter = BlogPostFrontMatter;
	fn content() -> IncludeDir {
		include_dir!("content/blog")
	}
}

pub struct ContentItem<T> {
	pub path: PathBuf,
	pub slug: String,
	pub front_matter: T,
	pub markdown: ui::Markdown,
}

pub trait Content: Sized {
	type FrontMatter: serde::de::DeserializeOwned;
	fn content() -> IncludeDir;

	fn slugs() -> Result<Vec<String>> {
		let content = Self::content();
		let slug_and_paths = content
			.into_iter()
			.map(|(entry, _)| {
				entry
					.parent()
					.unwrap()
					.file_name()
					.unwrap()
					.to_str()
					.unwrap()
					.to_owned()
			})
			.collect::<Vec<_>>();
		Ok(slug_and_paths)
	}

	fn list() -> Result<Vec<ContentItem<Self::FrontMatter>>> {
		Self::slugs()?
			.into_iter()
			.map(Self::from_slug)
			.collect::<Result<Vec<_>>>()
	}

	fn from_slug(slug: String) -> Result<ContentItem<Self::FrontMatter>> {
		let post_path = Path::new(&slug).join("post.md");
		let post = Self::content().read(&post_path).unwrap().data();
		let post = std::str::from_utf8(&post)?.to_owned();
		let (front_matter, post_markdown) = parse_and_find_content(&post)?;
		let front_matter = serde_yaml::from_reader(front_matter)?;
		let markdown = ui::Markdown::new(post_markdown);
		let ret = ContentItem {
			path: post_path,
			slug,
			front_matter,
			markdown,
		};
		Ok(ret)
	}
}

fn find_yaml_block(text: impl AsRef<[u8]>) -> Option<(usize, usize, usize)> {
	let text = text.as_ref();
	let text = std::str::from_utf8(&text).ok()?;
	let marker = "---\n";
	let marker_len = marker.len();
	match text.starts_with(marker) {
		true => {
			let slice_after_marker = &text[4..];
			let front_matter_end = slice_after_marker.find(marker)?;
			Some((
				marker_len,
				front_matter_end + marker_len,
				front_matter_end + 2 * marker_len,
			))
		}
		false => None,
	}
}

pub fn parse_and_find_content(text: &str) -> Result<(impl std::io::Read + '_, String)> {
	match find_yaml_block(text) {
		Some((fm_start, fm_end, content_start)) => {
			let yaml_str = &text[fm_start..fm_end];
			let front_matter = std::io::Cursor::new(yaml_str);
			let rest_of_text = text[content_start..].to_string();
			Ok((front_matter, rest_of_text))
		}
		None => Err(anyhow!("Invalid YAML frontmatter")),
	}
}
