use std::process::{Command, ExitStatus, Output};
use std::env;
use std::path::Path;
use std::fs;

#[cfg(not(target_os = "windows"))]
use std::os::unix::process::ExitStatusExt;

#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt;

fn main() {
	copy_resources();

	println!("cargo:rerun-if-changed=src/");
	let output = String::from_utf8(
		Command::new("git")
			.args(["rev-parse", "HEAD"])
			.output()
			.unwrap_or(Output {
				stdout: vec![],
				stderr: vec![],
				status: ExitStatus::from_raw(0),
			})
			.stdout,
	)
	.unwrap_or_default();
	let git_hash = if output == String::default() { "dev".into() } else { output };
	println!("cargo:rustc-env=GIT_HASH={git_hash}");
}

fn copy_resources() {
	// Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    
    // Calculate the final binary location
    // This will be target/debug or target/release depending on the build
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(4)
        .unwrap()
        .join(&profile)
		.join("static");

    // Copy resources
    copy_dir_recursive("static", &target_dir).unwrap();
    
    // Tell cargo to rerun this script if resources directory changes
    println!("cargo:rerun-if-changed=resources");
}

fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    if !dst.as_ref().exists() {
        fs::create_dir_all(&dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(entry.path(), dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }

    Ok(())
}
