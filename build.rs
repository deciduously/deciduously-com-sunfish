// Generates the blog templates and Rust module

#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pulldown_cmark::{self, html};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

const MODULE_NAME: &str = "blog";

// Compiles drafts to templates and generates struct
#[derive(Parser)]
#[grammar = "draft.pest"]
struct Draft;

#[derive(Debug, Default, Clone)]
pub struct BlogPost {
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub edited: Option<String>, // only if published
    pub id: usize,
    pub published: bool,
    pub markdown: String,
    pub url_name: String,
    pub tags: String, // TODO Vec<String>
    pub title: String,
}

impl BlogPost {
    fn new(id: usize, path: PathBuf) -> Self {
        // Init empty post
        let mut ret = Self::default();
        ret.id = id;
        ret.url_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        // fill in struct from draft
        let md_file = fs::read_to_string(path.to_str().unwrap()).expect("Could not read draft");
        let parse_tree = Draft::parse(Rule::draft, &md_file)
            .expect("Could not parse draft")
            .next()
            .unwrap();
        // cycle through each attribute
        // unwrap is safe - if it parsed, there are between 3 and 6
        let mut parse_tree_inner = parse_tree.into_inner();

        // set header
        let header = parse_tree_inner.next().unwrap();
        let attributes = header.into_inner();
        for attr in attributes {
            let mut name: &str = "";
            let mut value: &str = "";
            for attr_part in attr.into_inner() {
                match attr_part.as_rule() {
                    Rule::key => name = attr_part.as_str(),
                    Rule::value => value = attr_part.as_str(),
                    _ => unreachable!(),
                }
            }
            match name {
                "cover_image" => ret.cover_image = Some(value.to_string()),
                "description" => ret.description = Some(value.to_string()),
                "edited" => ret.edited = Some(value.to_string()),
                "published" => {
                    ret.published = match value {
                        "true" => true,
                        _ => false,
                    }
                }
                "tags" => ret.tags = value.to_string(),
                "title" => ret.title = value.to_string(),
                _ => {}
            }
        }

        // set body
        let body = parse_tree_inner.next().unwrap();
        ret.markdown = body.as_str().to_string();

        // done
        ret
    }
    fn write_template(&self) -> Result<(), io::Error> {
        let mut file = fs::File::create(&format!("templates/post_{}.html", self.url_name))?;
        let parser = pulldown_cmark::Parser::new(&self.markdown);
        let mut html = String::new();
        html::push_html(&mut html, parser);
        writeln!(file, "{{#  This file was auto-generated by build.rs #}}")?;
        writeln!(file, "{{% extends \"skel.html\" %}}")?;
        writeln!(file, "{{% block title %}}{}{{% endblock %}}", self.title)?;
        writeln!(
            file,
            "{{% block content %}}<main>{}</main>{{% endblock %}}",
            html
        )?;
        Ok(())
    }
    fn handler_name(&self) -> String {
        self.url_name
            .chars()
            .map(|c| if c == '-' { '_' } else { c })
            .collect()
    }
    fn struct_name(&self) -> String {
        format!("Blog{}Template", self.id)
    }
    fn write_template_struct(&self, file: &mut fs::File) -> Result<(), io::Error> {
        writeln!(file, "#[derive(Template)]")?;
        writeln!(file, "#[template(path = \"post_{}.html\")]", self.url_name)?;
        writeln!(file, "pub struct {} {{\n", &self.struct_name())?;
        writeln!(file, "    links: &'static [Hyperlink],")?;
        writeln!(file, "}}")?;
        writeln!(file, "impl Default for {} {{", &self.struct_name())?;
        writeln!(file, "    fn default() -> Self {{")?;
        writeln!(file, "        Self {{ links: &NAV }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
        Ok(())
    }
    fn write_handler(&self, file: &mut fs::File) -> Result<(), io::Error> {
        writeln!(
            file,
            "pub async fn {}() -> HandlerResult {{",
            self.handler_name()
        )?;
        writeln!(file, "    string_handler(")?;
        writeln!(file, "        &{}::default()", self.struct_name())?;
        writeln!(file, "            .render()")?;
        writeln!(file, "            .expect(\"Should render markup\"),")?;
        writeln!(file, "        \"text/html\",")?;
        writeln!(file, "        None,")?;
        writeln!(file, "    )")?;
        writeln!(file, "    .await")?;
        writeln!(file, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Blog {
    pub drafts: Vec<BlogPost>,
    pub published: Vec<BlogPost>,
}

impl Blog {
    fn new() -> Self {
        let mut ret = Blog::default();
        // scrape posts
        let paths = fs::read_dir("blog").expect("Should locate blog directory");
        for path in paths {
            let path = path.expect("Could not open draft").path();
            let post = BlogPost::new(ret.total(), path);
            if post.published {
                ret.published.push(post);
            } else {
                ret.drafts.push(post);
            }
        }
        ret
    }
    fn total(&self) -> usize {
        self.drafts.len() + self.published.len()
    }
}

/*
fn write_blog_post_type(file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(file, "// This module was auto-generated by build.rs\n")?;
    writeln!(file, "#[derive(Debug, Default, Clone)]")?;
    writeln!(file, "pub struct BlogPost {{")?;
    writeln!(file, "    pub cover_image: Option<String>,")?;
    writeln!(file, "    pub description: Option<String>,")?;
    writeln!(file, "    pub edited: Option<String>,")?;
    writeln!(file, "    pub id: usize,")?;
    writeln!(file, "    pub published: bool,")?;
    writeln!(file, "    pub markdown: String,")?;
    writeln!(file, "    pub url_name: String,")?;
    writeln!(file, "    pub tags: String,")?;
    writeln!(file, "    pub title: String,")?;
    writeln!(file, "}}\n")?;
    Ok(())
}
*/

fn write_link_info_type(file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(file, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(file, "pub struct LinkInfo {{")?;
    writeln!(
        file,
        "    pub handler: fn() -> Box<dyn std::future::Future<Output=HandlerResult>>,"
    )?;
    writeln!(file, "    pub id: usize,")?;
    writeln!(file, "    pub url_name: &'static str,")?;
    writeln!(file, "    pub title: &'static str,")?;
    writeln!(file, "}}\n")?;
    Ok(())
}

fn write_blog_link_info_type(file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(file, "#[derive(Debug, Default)]")?;
    writeln!(file, "pub struct BlogLinkInfo {{")?;
    writeln!(file, "    pub drafts: Vec<LinkInfo>,")?;
    writeln!(file, "    pub published: Vec<LinkInfo>,")?;
    writeln!(file, "}}\n")?;
    Ok(())
}

fn generate_posts(blog: &Blog, file: &mut fs::File) -> Result<(), io::Error> {
    for p in &blog.drafts {
        p.write_template()?;
        p.write_handler(file)?;
    }
    for p in &blog.published {
        p.write_template()?;
        p.write_handler(file)?;
    }
    Ok(())
}

fn generate_blog_link_info(blog: &Blog, file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(file, "lazy_static! {{")?;
    writeln!(file, "    pub static ref LINKINFO: BlogLinkInfo = {{")?;
    writeln!(file, "        let mut ret = BlogLinkInfo::default();")?;
    for p in &blog.drafts {
        writeln!(file, "    ret.drafts.push(LinkInfo {{")?;
        writeln!(file, "        handler: {},", p.handler_name())?;
        writeln!(file, "        id: {},", p.id)?;
        writeln!(file, "        title: \"{}\",", p.title)?;
        writeln!(file, "        url_name: \"{}\",", p.url_name)?;
        writeln!(file, "    }});")?;
    }
    for p in &blog.published {
        writeln!(file, "    ret.published.push(LinkInfo {{")?;
        writeln!(file, "        handler: {},", p.handler_name())?;
        writeln!(file, "        id: {},", p.id)?;
        writeln!(file, "        title: \"{}\",", p.title)?;
        writeln!(file, "        url_name: \"{}\",", p.url_name)?;
        writeln!(file, "    }});")?;
    }
    writeln!(file, "        ret")?;
    writeln!(file, "    }};\n}}\n")?;
    Ok(())
}

fn generate_template_structs(blog: &Blog, file: &mut fs::File) -> Result<(), io::Error> {
    for p in &blog.drafts {
        p.write_template_struct(file)?;
    }
    for p in &blog.published {
        p.write_template_struct(file)?;
    }
    Ok(())
}

fn write_imports(file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(file, "// this module was auto-generated by build.rs")?;
    writeln!(file, "use askama::Template;")?;
    writeln!(file, "use crate::{{")?;
    writeln!(file, "    config::NAV,")?;
    writeln!(file, "    handlers::{{HandlerResult, string_handler}},")?;
    writeln!(file, "    types::Hyperlink,")?;
    writeln!(file, "}};")?;
    writeln!(file, "use lazy_static::lazy_static;")?;
    writeln!(file, "")?;
    Ok(())
}

fn generate_module(blog: &Blog) -> Result<(), io::Error> {
    let mut module = fs::File::create(&format!("src/{}.rs", MODULE_NAME))?;
    // use statements
    write_imports(&mut module)?;
    // types
    write_link_info_type(&mut module)?;
    write_blog_link_info_type(&mut module)?;
    // links
    generate_blog_link_info(blog, &mut module)?;
    // template structs
    generate_template_structs(blog, &mut module)?;
    // templates
    generate_posts(blog, &mut module)?;
    Ok(())
}

fn generate(blog: &Blog) -> Result<(), io::Error> {
    generate_module(blog)?;
    Ok(())
}

fn main() {
    let blog = Blog::new();
    println!("cargo:rerun-if-changed=blog");
    for p in &blog.drafts {
        println!("cargo:rerun-if-changed=blog/{}.md", p.url_name);
    }
    for p in &blog.published {
        println!("cargo:rerun-if-changed=blog/{}.md", p.url_name);
    }
    if let Err(e) = generate(&blog) {
        eprintln!("Error: {}", e);
    }
}
