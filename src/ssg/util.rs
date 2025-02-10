#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

use std::error::Error;
use std::fs::{copy, create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::io;

/// Resolve output path for a file being processed
pub fn output_path(
    input_path: &PathBuf,
    out_dir: &PathBuf,
    extension: String,
) -> Result<PathBuf, Box<dyn Error>> {
    //println!("Resolving output path for {}", input_path.display());

    Ok(out_dir
        .join(input_path.file_stem().expect("invalid filename"))
        .with_extension(extension))
}

/// Copy directory recursively
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // Check if source directory exists before trying to copy
    if !src.as_ref().exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source directory does not exist: {}", src.as_ref().display())
        ));
    }

    // Create destination directory and all parent directories
    create_dir_all(&dst)?;

    //println!("Copying {} to {}", src.as_ref().display(), dst.as_ref().display());
    for entry in read_dir(src)? {
        //println!("Copying entry: {:?}", entry);
        if entry.is_err() {
            eprintln!("Error reading entry: {:?}", entry);
            continue;
        }
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
