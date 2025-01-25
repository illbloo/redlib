#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

use std::error::Error;
use std::fmt;

use crate::bdfr::{to_comments, SubmissionArchiveEntry};
use crate::post::{comment_query, PostTemplate};
use crate::subreddit::SubredditTemplate;
use crate::utils::{Post, Preferences, Subreddit};

use clap::ValueEnum;
use serde_json::Value as JsonValue;

pub trait PostTemplater {
    fn template(&self) -> PostTemplate;
}

impl PostTemplater for SubmissionArchiveEntry {
    fn template(&self) -> PostTemplate {
        PostTemplate::new(
            self.to_post().unwrap(),
            to_comments(self.comments.clone()),
            "new".to_string(),
            Preferences::default(),
            true,
            self.url.clone(),
            comment_query(&self.url),
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum InputFormat {
    /// Inputs are JSON text posts created using Serene-Arc/bulk-downloader-for-reddit.
    BDFRSelfPost,
    /// Inputs are JSON posts from Reddit's API.
    RedditJson,
}

impl InputFormat {
    pub fn json_decode(&self, json: JsonValue) -> Result<impl PostTemplater, Box<dyn Error>> {
        match self {
            InputFormat::BDFRSelfPost => {
                Ok(serde_json::from_value::<SubmissionArchiveEntry>(json)?)
            },
            InputFormat::RedditJson => {
                todo!()
            }
        }
    }
}

impl fmt::Display for InputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            InputFormat::BDFRSelfPost => "bdfr-self-post",
            InputFormat::RedditJson => "reddit-json",
        };
        write!(f, "{:?}", s)
    }
}

/// Generate a SubredditTemplate for a static archive.
pub fn create_subreddit(
    posts: Vec<Post>,
    title: &str,
    description: &str,
    prefs: Preferences,
) -> Result<SubredditTemplate, Box<dyn Error>> {
    println!("Creating subreddit template with {} posts", posts.len());

    let no_posts = posts.is_empty();

    Ok(SubredditTemplate {
        sub: Subreddit {
            name: "redlib".to_string(),
            title: title.to_string(),
            description: description.to_string(),
            members: (String::new(), String::new()),
            info: String::new(),
            icon: String::new(),
            active: (String::new(), String::new()),
            wiki: false,
            nsfw: false,
        },
        url: "example.com".to_string(),
        posts,
        sort: (String::new(), String::new()),
        ends: (String::new(), String::new()),
        prefs,
        redirect_url: "redirect.example.com".to_string(),
        is_filtered: false,
        all_posts_filtered: false,
        all_posts_hidden_nsfw: false,
        no_posts,
    })
}