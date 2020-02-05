use lazy_static::lazy_static;
use log::*;
use pest::Parser;
use serde_derive::Deserialize;
use std::{fmt, fs, path::PathBuf};

//
// General
//

/// Link icon do display in header
#[derive(Default, Deserialize, Debug, Clone)]
pub struct Hyperlink {
    pub name: String,
    pub target: String,
}

impl Hyperlink {
    pub fn new(name: &str, target: &str) -> Self {
        Self {
            name: name.into(),
            target: target.into(),
        }
    }
}

//
// Blog
//

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
    pub tags: String,     // TODO Vec<String>
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
                _ => error!("Unknown attribute {}!", name),
            }
        }

        // set body
        let body = parse_tree_inner.next().unwrap();
        ret.markdown = body.as_str().to_string();

        println!("{:?}", ret);
        ret
    }
    fn get_template(&self) -> String {
        unimplemented!()
    }
}

lazy_static! {
    pub static ref BLOG: Blog = Blog::new();
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
            debug!("Adding path {:?}", path);
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

//
// Resume/CV
//

/// Link icon do display in header
#[derive(Default, Deserialize, Debug)]
pub struct CvLink {
    pub hyperlink: Hyperlink,
    pub icon: String,
}

/// Schema.org locality
#[derive(Default, Deserialize, Debug)]
pub struct Locality {
    pub name: String,
    pub postal_code: String,
    pub state: AddressRegion,
}

/// Schema.org addressRegion
#[derive(Default, Deserialize, Debug)]
pub struct AddressRegion {
    pub abbreviation: String,
    pub full_name: String,
    pub country: String,
}

/// Schema.org Postal Address
#[derive(Default, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub line2: Option<String>,
    pub locality: Locality,
}

/// Month
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Month {
    Jan = 1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Default for Month {
    fn default() -> Self {
        Self::Jan
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Date
#[derive(Default, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct MonthYear {
    pub year: i16,
    pub month: Month,
}

/// Type of degree
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum DegreeType {
    Cert,
    BS,
}

impl DegreeType {
    pub fn full_name(&self) -> &str {
        use DegreeType::*;
        match self {
            Cert => "Certificate",
            BS => "Bachelor of Science",
        }
    }
    pub fn abbreviation(&self) -> &str {
        use DegreeType::*;
        match self {
            Cert => "Cert.",
            BS => "B.S.",
        }
    }
}

impl Default for DegreeType {
    fn default() -> Self {
        Self::BS
    }
}

/// Single degree
#[derive(Default, Deserialize, Debug)]
pub struct Degree {
    pub degree_type: DegreeType,
    pub graduation_date: MonthYear,
    pub expected: bool,
    pub gpa: f32,
    pub subject: String,
}

/// Education entry
#[derive(Default, Deserialize, Debug)]
pub struct School {
    pub name: String,
    pub address: Address,
    pub degrees: Vec<Degree>,
}

/// Intro
#[derive(Default, Deserialize, Debug)]
pub struct Intro {
    pub one_liner: String,
    pub about: String,
    pub skills: String,
    pub techs: String,
}

/// CV Header
#[derive(Default, Deserialize, Debug)]
pub struct CvHeader {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub subtitle: String,
    pub links: Vec<CvLink>,
    pub address: Address,
}

impl CvHeader {
    pub fn mail_to(&self) -> String {
        format!("mailto:{}", self.email)
    }
}

/// Projects
#[derive(Default, Deserialize, Debug)]
pub struct CvProject {
    pub name: String,
    pub synopsis: String,
}

/// Employment Entry
#[derive(Default, Deserialize, Debug)]
pub struct CvEmployment {
    pub title: String,
    pub employer: String,
    pub begin_date: MonthYear,
    pub end_date: Option<MonthYear>,
    pub current: bool,
    pub address: Address,
    pub bullets: Vec<String>,
}

/// CV/Resume
#[derive(Default, Deserialize, Debug)]
pub struct CV {
    pub header: CvHeader,
    pub education: Vec<School>,
    pub intro: Intro,
    pub projects: Vec<CvProject>,
    pub employment: Vec<CvEmployment>,
}
