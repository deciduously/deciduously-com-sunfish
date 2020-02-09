---
cover_image: https://thepracticaldev.s3.amazonaws.com/i/qy9lgsrochopihygaxt0.png
edited: 2018-11-15T12:00:00.000Z
title: Scrape your Dev.to pageviews with Rust
published: true
description: A quick webscraper to generate CSVs from your pageviews
tags: rust, beginners
---
Here's a quick 'n' dirty way to dump your new-fangled post analytics to a CSV using Rust.  You have to save the page source to `src/page.html`.  Y'know, for graphs and stuff.  Who doesn't like graphs?

This ain't polished - It was my "one-hour-before-my-day-job-starts" project today.  Snag the regex for your own real version, or improve this one and show me!


```rust
extern crate chrono;
extern crate csv;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate select;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use regex::Regex;
use select::{
    document::Document,
    predicate::{Class, Name},
};
use std::{
    error::Error,
    fs::{File, OpenOptions},
};

lazy_static! {
    static ref NOW: DateTime<Local> = Local::now();
    static ref STAT_RE: Regex = Regex::new(".+?([0-9]+).+//.?([0-9]+).+//.?([0-9]+).+").unwrap();
}

#[derive(Debug, Serialize)]
struct Record {
    time: String,
    title: String,
    views: i32,
    reactions: i32,
    comments: i32,
}

impl Record {
    fn new(time: String, title: String, views: i32, reactions: i32, comments: i32) -> Self {
        Self {
            time,
            title,
            views,
            reactions,
            comments,
        }
    }
}

fn write_entries(rs: Vec<Record>, f: File) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(f);
    for r in rs {
        wtr.serialize(r)?;
    }
    wtr.flush()?;
    Ok(())
}

fn scrape_page(doc: &Document) -> Result<Vec<Record>, Box<Error>> {
    let mut ret = Vec::new();
    for node in doc.find(Class("dashboard-pageviews-indicator")) {
        let text = node.text();
        if STAT_RE.is_match(&text) {
            let title = node
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .find(Name("a"))
                .next()
                .unwrap()
                .find(Name("h2"))
                .next()
                .unwrap()
                .text();
            for cap in STAT_RE.captures_iter(&text) {
                let r = Record::new(
                    NOW.to_rfc2822(),
                    title.clone(),
                    cap[1].parse::<i32>()?,
                    cap[2].parse::<i32>()?,
                    cap[3].parse::<i32>()?,
                );
                ret.push(r);
            }
        }
    }
    Ok(ret)
}

fn run() -> Result<(), Box<Error>> {
    let doc = Document::from(include_str!("page.html"));
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("stats.csv")?;
    let entries = scrape_page(&doc)?;
    write_entries(entries, file)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        ::std::process::exit(1);
    }
}


```

*edit* finished off the error handling
