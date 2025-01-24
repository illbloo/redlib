#![forbid(unsafe_code)]
#![allow(clippy::cmp_owned)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::{canonicalize, File};
use std::path::{Path, PathBuf};

use rinja::Template;

use crate::{
    post::PostTemplate,
    ssg::util::copy_dir_all,
};

/// Write templates to HTML files
pub async fn write_templates(templates: HashMap<PathBuf, PostTemplate>) -> Result<(), Box<dyn Error>> {
    for (path, tmpl) in templates {
        tmpl.write_into(&mut File::create(path)?)?;
    }
    Ok(())
}

/// Copy static files to the site output directory
pub async fn copy_static(out_dir: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    Ok(copy_dir_all(canonicalize("../../static")?, out_dir)?)
}