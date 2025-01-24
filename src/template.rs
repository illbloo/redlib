#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

use std::error::Error;
use std::fmt;

use crate::bdfr::SubmissionArchiveEntry;
use crate::post::{comment_query, PostTemplate};
use crate::utils::Preferences;

use clap::ValueEnum;
use serde_json::Value as JsonValue;

pub trait PostTemplater {
    fn template(&self) -> PostTemplate;
}

impl PostTemplater for SubmissionArchiveEntry {
    fn template(&self) -> PostTemplate {
        PostTemplate::new(
            self.to_post().unwrap(),
            self.comments.iter().map(|c| c.to_comment()).collect(),
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
