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

pub fn to_comments(entries: Vec<CommentArchiveEntry>) -> Vec<Comment> {
    entries.iter().map(|entry| entry.to_comment()).collect()
}

impl SubmissionArchiveEntry {
    /// Create a SubmissionArchiveEntry from a Post
    pub fn from_post(post: &Post, comments: &Vec<Comment>) -> Self {
        SubmissionArchiveEntry {
            title: post.title.clone(),
            name: post.id.clone(),
            url: post.ws_url.clone(),
            selftext: post.body.clone(),
            score: post.score.0.parse().unwrap_or(0),
            upvote_ratio: post.upvote_ratio as f64,
            permalink: post.permalink.clone(),
            id: post.id.clone(),
            author: post.author.name.clone(),
            link_flair_text: post.flair.text.clone(),
            num_comments: comments.len().try_into().unwrap_or(0),
            over_18: post.flags.nsfw.clone(),
            spoiler: post.flags.spoiler.clone(),
            pinned: post.flags.stickied.clone(),
            locked: false,
            distinguished: Some(post.author.distinguished.clone()),
            created_utc: post.created.parse().unwrap_or(0.0),
            comments: comments
                .iter()
                .map(|reply| CommentArchiveEntry::from_comment(reply))
                .collect(),
        }
    }

    /// Convert to a Post (for PostTemplate)
    pub fn to_post(&self) -> Result<Post, Box<dyn Error>> {
        println!("Creating Post from SubmissionArchiveEntry");
        let media = Media {
            url: self.url.clone(),
            alt_url: self.url.clone(),
            width: 0,
            height: 0,
            poster: self.author.clone(),
            download_name: "asdfasdfsd".to_string(),
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
            community: "".to_string(),
            author: Author {
                name: self.author.clone(),
                flair: Flair {
                    text: "".to_string(),
                    flair_parts: Vec::new(),
                    background_color: "".to_string(),
                    foreground_color: "".to_string(),
                },
                distinguished: "".to_string(),
            },
            link_title: self.link_flair_text.clone(),
            poll: None,
            post_type: "link".to_string(),
            flair: Flair {
                text: self.link_flair_text.to_string(),
                flair_parts: Vec::new(),
                background_color: "".to_string(),
                foreground_color: "".to_string(),
            },
            flags: Flags {
                nsfw: self.over_18.clone(),
                spoiler: self.spoiler.clone(),
                stickied: self.pinned.clone(),
            },
            thumbnail: media.clone(),
            media: media.clone(),
            domain: "".to_string(),
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
}

/// BDFR representation of a Comment (a reply in a Reddit thread).
#[derive(Clone, Deserialize)]
pub struct CommentArchiveEntry {
	pub author: String,
	pub id: String,
	pub score: i64,
	pub author_flair: Option<String>,
	pub submission: String,
	pub stickied: bool,
	pub body: String,
	pub is_submitter: bool,
	pub distinguished: Option<String>,
	pub created_utc: f64,
	pub parent_id: String,
	pub replies: Vec<CommentArchiveEntry>,
}

impl CommentArchiveEntry {
    /// Create from a Comment object
    pub fn from_comment(comment: &Comment) -> Self {
        CommentArchiveEntry {
            author: comment.author.name.clone(),
            id: comment.id.clone(),
            score: comment.score.0.parse().unwrap_or(0),
            author_flair: Some(comment.author.flair.text.clone()),
            submission: comment.parent_id.clone(),
            stickied: comment.highlighted.clone(),
            body: comment.body.clone(),
            is_submitter: comment.post_author == comment.author.name,
            distinguished: Some(comment.author.distinguished.clone()),
            created_utc: comment.created.parse().unwrap_or(0.0),
            parent_id: comment.parent_id.clone(),
            replies: comment.replies
                .iter()
                .map(|reply| CommentArchiveEntry::from_comment(reply))
                .collect(),
        }
    }

    /// Convert to a Comment, for PostTemplate
    pub fn to_comment(&self) -> Comment {
        let body = self.body.clone();

        Comment {
            id: self.id.clone(),
            kind: ThingKind::Comment.to_string(),
            parent_id: self.parent_id.clone(),
            parent_kind: ThingKind::from_fullname(&self.parent_id).expect("parent_id must contain thing id (t1_, t2_, etc)").to_string(),
            post_link: "".to_string(),
            post_author: self.is_submitter.to_string(),
            body,
            author: Author {
                name: self.author.clone(),
                flair: Flair {
                    text: self.author_flair.clone().unwrap_or_else(|| "".to_string()),
                    flair_parts: Vec::new(),
                    background_color: "".to_string(),
                    foreground_color: "".to_string(),
                },
                distinguished: self.distinguished.clone().unwrap_or("".to_string()),
            },
            score: (self.score.to_string(), "".to_string()),
            rel_time: self.created_utc.to_string(),
            created: self.created_utc.to_string(),
            edited: ("".to_string(), "".to_string()),
            replies: self.replies.iter().map(|reply| reply.to_comment()).collect(),
            highlighted: self.stickied,
            awards: Vec::new(),
            collapsed: false,
            is_filtered: false,
            more_count: 0,
            prefs: Preferences::default(),
        }
    }
}
