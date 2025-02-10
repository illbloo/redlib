#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

// Models for Serene-Arc/bulk-downloader-for-reddit
// https://github.com/Serene-Arc/bulk-downloader-for-reddit

use crate::models::ThingKind;
use crate::utils::{format_selftext, Author, Comment, Flags, Flair, Media, Post, Preferences};

use std::error::Error;
use serde::Deserialize;

/// BDFR representation of a Post (the original post in a Reddit thread).
#[derive(Clone, Deserialize)]
pub struct SubmissionArchiveEntry {
    // Post title
	pub title: String,
    /// Fullname of the post (e.g. "t3_abcdef")
	pub name: String,
    /// Original URL of the post on Reddit
	pub url: String,
    /// Post text body
	pub selftext: String,
    /// Upvotes minus downvotes
	pub score: i64,
    /// Upvote-to-downvote ratio
	pub upvote_ratio: f64,
    /// Link to the post within the archive
	pub permalink: String,
	pub id: String,
    /// Post author's Reddit username
	pub author: String,
    /// Flair text attached to the post
	pub link_flair_text: String,
    /// Number of comments on the post, including replies to other comments
	pub num_comments: i64,
    /// Whether the post is marked as NSFW
	pub over_18: bool,
    /// Whether the post is marked as containing spoilers
	pub spoiler: bool,
    /// Whether the post is pinned to the subreddit
	pub pinned: bool,
    /// Whether the post's comments section was locked the subreddit's mods
	pub locked: bool,
	pub distinguished: Option<String>,
    /// Unix timestamp (UTC) of when the post was created
	pub created_utc: f64,
    /// Post comments, threaded
	pub comments: Vec<CommentArchiveEntry>,
}

impl SubmissionArchiveEntry {
    /// Convert to a Post (for PostTemplate)
    pub fn to_post(&self) -> Result<Post, Box<dyn Error>> {
        println!("Creating Post from SubmissionArchiveEntry");
        let media = Media {
            url: self.url.clone(),
            alt_url: self.url.clone(),
            width: 0,
            height: 0,
            poster: self.author.clone(),
            download_name: String::new(),
        };

        let created = self.created_utc.to_string();

        Ok(Post {
            title: self.title.clone(),
            ws_url: self.url.clone(),
            body: format_selftext(&self.selftext),
            score: (self.score.to_string(), "".to_string()),
            upvote_ratio: self.upvote_ratio as i64,
            permalink: self.permalink.clone(),
            id: self.id.clone(),
            community: String::new(),
            author: Author {
                name: self.author.clone(),
                flair: Flair::default(),
                distinguished: String::new(),
            },
            link_title: self.link_flair_text.clone(),
            poll: None,
            post_type: "link".to_string(),
            flair: Flair::default(),
            flags: Flags {
                nsfw: self.over_18.clone(),
                spoiler: self.spoiler.clone(),
                stickied: self.pinned.clone(),
            },
            thumbnail: media.clone(),
            media: media.clone(),
            domain: self.url.clone(),
            rel_time: created.clone(),
            created,
            created_ts: self.created_utc.clone() as u64,
            num_duplicates: 0,
            comments: (String::new(), String::new()),
            gallery: Vec::new(),
            awards: Vec::new(),
            nsfw: self.over_18.clone(),
            out_url: None,
        })
    }

    pub fn comments(&self) -> Vec<Comment> {
        self.comments.iter().map(|c| c.to_comment(&self)).collect()
    }
}

/// BDFR representation of a Comment (a reply in a Reddit thread).
#[derive(Clone, Deserialize)]
pub struct CommentArchiveEntry {
    /// Comment author's Reddit username
	pub author: String,
    /// ID of the comment
	pub id: String,
    /// Comment score (upvotes minus downvotes)
	pub score: i64,
	pub author_flair: Option<String>,
    /// ID of the original post in this thread
	pub submission: String,
    /// Whether the comment is pinned
	pub stickied: bool,
    /// Post contents
	pub body: String,
    /// If author is the original post's author
	pub is_submitter: bool,
	pub distinguished: Option<String>,
	pub created_utc: f64,
    /// Fullname ID of the post or parent this is replying too (e.g. "t1_abcdef")
	pub parent_id: String,
	pub replies: Vec<CommentArchiveEntry>,
}

impl CommentArchiveEntry {
    /// Convert to a Comment, for PostTemplate
    pub fn to_comment(&self, subm: &SubmissionArchiveEntry) -> Comment {
        Comment {
            id: self.id.clone(),
            kind: ThingKind::Comment.to_string(),
            parent_id: self.parent_id.clone(),
            parent_kind: ThingKind::from_fullname(&self.parent_id).expect("parent_id must contain thing id").to_string(),
            post_link: subm.permalink.clone(),
            post_author: subm.author.clone(),
            body: self.body.clone(),
            author: Author {
                name: self.author.clone(),
                flair: Flair {
                    text: self.author_flair.clone().unwrap_or_else(|| String::new()),
                    flair_parts: Vec::new(),
                    background_color: String::new(),
                    foreground_color: String::new(),
                },
                distinguished: self.distinguished.clone().unwrap_or(String::new()),
            },
            score: (self.score.to_string(), String::new()),
            rel_time: strtime(self.created_utc as i64).unwrap_or_else(|_| self.created_utc.to_string()),
            created: String::new(),
            edited: (String::new(), String::new()),
            replies: self.replies.iter().map(|reply| reply.to_comment(&subm)).collect(),
            highlighted: self.stickied,
            awards: Vec::new(),
            collapsed: false,
            is_filtered: false,
            more_count: 0,
            prefs: Preferences::default(),
        }
    }
}

fn strtime(timestamp: i64) -> Result<String, Box<dyn Error>> {
    Ok(time::OffsetDateTime::from_unix_timestamp(timestamp)?
        .format(&time::format_description::well_known::Rfc2822)?)
}