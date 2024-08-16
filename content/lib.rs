use anyhow::{anyhow, Result};
use deciduously_com_sunfish_ui as ui;
use std::path::{Path, PathBuf};
use sunfish::{include_dir, include_dir::IncludeDir};
use url::Url;

pub struct BlogPost;

#[derive(serde::Deserialize, PartialEq, Eq)]
pub struct BlogPostFrontMatter {
	pub cover_image: Option<Url>,
	#[serde(with = "time::serde::rfc3339")]
	pub date: time::OffsetDateTime,
	pub description: Option<String>,
	pub tags: Option<Vec<String>>,
	pub title: String,
}

impl Content for BlogPost {
	type FrontMatter = BlogPostFrontMatter;
	fn content() -> IncludeDir {
		include_dir!("content/blog")
	}
}

impl PartialOrd for BlogPostFrontMatter {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for BlogPostFrontMatter {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.date.cmp(&other.date)
	}
}

#[derive(PartialEq, Eq)]
pub struct ContentItem<T: Ord> {
	pub path: PathBuf,
	pub slug: String,
	pub front_matter: T,
	pub markdown: ui::Markdown,
}

impl<T: Ord> PartialOrd for ContentItem<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<T: Ord> Ord for ContentItem<T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.front_matter.cmp(&other.front_matter)
	}
}

pub trait Content: Sized {
	type FrontMatter: serde::de::DeserializeOwned + Ord;
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
		let mut list = Self::slugs()?
			.into_iter()
			.map(Self::from_slug)
			.collect::<Result<Vec<_>>>()?;
		list.sort();
		list.reverse();
		Ok(list)
	}

	fn from_slug(slug: String) -> Result<ContentItem<Self::FrontMatter>> {
		let post_path = Path::new(&slug).join("post.md");
		let post = Self::content().read(&post_path).unwrap().data();
		let post_str = std::str::from_utf8(&post)?.to_owned();
		let (front_matter, markdown) = parse_and_find_content(&post_str)?;
		let front_matter = serde_yaml::from_reader(front_matter)?;
		let ret = ContentItem {
			path: post_path,
			slug,
			front_matter,
			markdown,
		};
		Ok(ret)
	}
}

fn find_yaml_block(text: &str) -> Option<(usize, usize, usize)> {
	let marker = "---\n";
	let marker_len = marker.len();
	match text.starts_with(marker) {
		true => {
			let slice_after_marker = &text[marker_len..];
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

pub fn parse_and_find_content(text: &str) -> Result<(impl std::io::Read + '_, ui::Markdown)> {
	// Here is where you could search for TOML `+++` or JSON `{`
	match find_yaml_block(text) {
		Some((front_matter_start, front_matter_end, content_start)) => {
			let yaml_str = &text[front_matter_start..front_matter_end];
			let front_matter = yaml_str.as_bytes();
			let post = text[content_start..].to_string();
			let post = ui::Markdown::new(post);
			Ok((front_matter, post))
		}
		None => Err(anyhow!("Invalid YAML frontmatter")),
	}
}
