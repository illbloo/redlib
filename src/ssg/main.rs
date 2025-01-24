#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::{canonicalize, create_dir_all, read_dir, File};
use std::path::{Path, PathBuf};

use clap::Parser;
use redlib::ssg::util::output_path;
use redlib::template::PostTemplater;
use serde_json::Value;

use redlib::{
    post::PostTemplate,
    template::InputFormat,
    ssg::writer::{copy_static, write_templates},
};

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

    println!("Indexing input files...");
    let paths = json_paths_recursive(&src_path)?;

    println!("Creating templates...");
    let tmpls = create_templates(paths, &out_dir, &config.input_format)?;

    println!("Copying static files...");
    copy_static(&out_dir).await?;

    println!("Writing site files...");
    write_templates(tmpls).await?;

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
fn create_templates(
    paths: Vec<PathBuf>,
    out_dir: &PathBuf,
    input_format: &InputFormat,
) -> Result<HashMap<PathBuf, PostTemplate>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for input_path in paths {
        let (k, v) = create_template(&input_path, &out_dir, &input_format)?;
        map.insert(k, v);
    }

    Ok(map)
}

/// Create a PostTemplate from a JSON file
fn create_template(
    input_path: &PathBuf,
    out_dir: &PathBuf, 
    input_format: &InputFormat,
) -> Result<(PathBuf, PostTemplate), Box<dyn Error>> {
    println!("Creating template for {}", input_path.display());
    
    // Resolve output path
    println!("Resolving output path");
    let out_path = output_path(input_path, out_dir, "html".to_string())?;

    // Read JSON file
    println!("Reading JSON data");
    let json: Value = serde_json::from_reader(File::open(input_path)?)?;

    // Build the PostTemplate
    println!("Creating PostTemplate");
    let tmpl = InputFormat::json_decode(input_format, json)?.template();

    Ok((out_path, tmpl))
}
