#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::{canonicalize, create_dir_all, read_dir, File};
use std::path::{Path, PathBuf};

use clap::Parser;
use serde_json::Value;

use redlib::bdfr::SubmissionArchiveEntry;
use redlib::ssg::{
    template::{InputFormat, create_subreddit},
    util::output_path,
    writer::write_all,
};
use redlib::post::PostTemplate;
use redlib::utils::{Comment, Post, Preferences};

/// Config for the generator (as well as the CLI parser itself)
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Generate static sites from Reddit archives",
    long_about = None,
)]
struct Cli {
    #[arg(
        short = 's',
        long = "source",
        value_name = "SOURCE",
        required = true,
        help = "Path to directory of JSON files to be parsed"
    )]
    source: String,

    #[arg(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        default_value = "out",
    )]
    output: String,

    #[arg(
        short = 'f',
        long = "input-format",
        value_name = "INPUT_FORMAT",
        help = "Format of files in the input directory.",
        default_value_t = InputFormat::BDFRSelfPost,
    )]
    #[arg(value_enum)]
    input_format: InputFormat,

    #[arg(
        short = 't',
        long = "title",
        value_name = "TITLE",
        help = "Title for the generated site",
        default_value = "Redlib Archive",
    )]
    archive_title: String,

    #[arg(
        short = 'd',
        long = "desc",
        value_name = "DESCRIPTION",
        help = "Description for the generated site",
        default_value = "An archive of Reddit posts.",
    )]
    archive_desc: String,
}

impl Cli {
    /// Get template preferences for this configuration
    pub fn template_prefs(&self) -> Preferences {
        let mut prefs = Preferences::default();
        prefs.static_path = "static".to_string();
        prefs.show_nsfw = "on".to_string();
        prefs.disable_visit_reddit_confirmation = "on".to_string();

        prefs
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables
	_ = dotenvy::dotenv();

	// Initialize logger
	pretty_env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Run the generator
    create_site(&cli).await.unwrap();
}

/// Run the site generator with a given config.
async fn create_site(config: &Cli) -> Result<(), Box<dyn Error>> {
    // Canonicalize paths
    let src_path = canonicalize(&config.source)?;
    let out_dir = PathBuf::try_from(&config.output)?;

    // Create output directory if it doesn't exist
    if !out_dir.exists() {
        create_dir_all(&config.output)?;
    }

    let prefs = config.template_prefs();

    println!("Indexing input files...");
    let paths = json_paths_recursive(&src_path)?;

    println!("Building posts...");
    let posts = create_posts(paths, &out_dir, &config.input_format)?;

    println!("Building subreddit page...");
    let sub = create_subreddit(
        posts.iter().map(|(_, (post, _))| post.clone()).collect(),
        &config.archive_title,
        &config.archive_desc,
        prefs.clone(),
    )?;

    println!("Building post templates...");
    let tmpls = build_post_templates(posts, prefs.clone())?;

    println!("Writing site files...");
    write_all(tmpls, sub, &config.output, prefs.static_path).await?;

    println!("Site generated at {}", &config.output);

    Ok(())
}

/// Resolve paths of all JSON files in a directory and its subdirectories
fn json_paths_recursive(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = Vec::new();

    for entry in read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() && !path.starts_with(".") {
            paths.extend(json_paths_recursive(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "json") {
            paths.push(path);
        }
    }

    Ok(paths)
}

/// Create PostTemplate objects from JSON files
fn create_posts(
    paths: Vec<PathBuf>,
    out_dir: &PathBuf,
    input_format: &InputFormat,
) -> Result<HashMap<PathBuf, (Post, Vec<Comment>)>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for input_path in paths {
        let (k, v) = create_post(&input_path, &out_dir, &input_format)?;
        map.insert(k, v);
    }

    Ok(map)
}

fn create_post(
    input_path: &PathBuf,
    out_dir: &PathBuf, 
    input_format: &InputFormat,
) -> Result<(PathBuf, (Post, Vec<Comment>)), Box<dyn Error>> {
    println!("Creating template for {}", input_path.display());
    
    // Resolve output path
    println!("Resolving output path");
    let out_path = output_path(input_path, out_dir, "html".to_string())?;

    // Read JSON file
    println!("Reading JSON data into post");
    let json: Value = serde_json::from_reader(File::open(input_path)?)?;

    Ok(match input_format {
        InputFormat::BDFRSelfPost => {
            let subm: SubmissionArchiveEntry = serde_json::from_value(json)?;
            let mut post = subm.to_post()?;
            post.permalink = out_path.file_name().unwrap().to_string_lossy().to_string();
            let comments = subm.comments();

            (out_path, (post, comments))
        },
        InputFormat::RedditJson => todo!(),
    })
}

fn build_post_templates(
    posts: HashMap<PathBuf, (Post, Vec<Comment>)>,
    prefs: Preferences,
) -> Result<HashMap<PathBuf, PostTemplate>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for (path, (post, comments)) in posts {
        map.insert(path, PostTemplate::new(
            post,
            comments,
            "new".to_string(),
            prefs.clone(),
            true,
            "".to_string(),
            "".to_string(),
        ));
    }

    Ok(map)
}
