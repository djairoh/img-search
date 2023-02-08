use std::borrow::Borrow;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit, Stdio};
use rustnao::HandlerBuilder;
use log::{debug, error, trace, warn};
use config::Config;
use clap::Parser;
use crate::cli::Cli;

mod image_download;
mod config;
mod cli;

/// This function removes temporary files.
///
/// arguments:
/// path: Option<PathBuf> - if Some, removes the given path, assumed to be a directory
/// files: Option<Vec<PathBuf>> - if Some, removes the files in the vec
fn cleanup(path: Option<&PathBuf>, files: Option<Vec<&PathBuf>>) {
    if let Some(files) = files {
        debug!("Removing {} file(s).", files.len());
        for file in files {
            trace!("Removing \"{}\".", file.display());
            if let Err(e) = Command::new("/usr/bin/rm")
                .arg(file.as_os_str())
                .stdout(Stdio::piped())
                .output() {
                warn!("Failed to execute rm: {}", e.to_string());
            }
        }
    }

    if let Some(path) = path {
        debug!("Removing directory \"{}\"", path.display());
        if let Err(e) = fs::remove_dir(path) {
            error!("Failed to remove directory: {}", e.to_string());
            exit(1);
        }
    }
}

/// main function
fn main() {
    //env variables and config
    let cfg: Config = confy::load("img-search", None).unwrap();
    std::env::set_var("RUST_LOG", cfg.rust_log);
    env_logger::init();

    //confirm input
    let mut args: Cli = Cli::parse();
    cli::check_input(&mut args);

    //setup handler
    let handle = HandlerBuilder::new()
        .api_key(cfg.api_key.borrow())
        .num_results(cfg.num_results)
        .min_similarity(cfg.min_similarity)
        .build();

    //get sauce
    let query = handle.get_sauce(args.file.to_str().expect(""), None, None);
    if let Err(e) = query {
        error!("Query failed: {}", e.kind());
        cleanup(Some(&args.dir),None);
        exit(1);
    }

    //download thumbnails for sauce
    let downloaded_images = image_download::download_images(query.unwrap(), &args.dir);
    for image in downloaded_images.iter() {
        println!("{}", image.display());
    }

    //open files in feh
    //because of the way .output() works, this automatically blocks the thread until feh closes
    let _ = Command::new("/usr/bin/feh")
        .arg(args.dir.as_os_str())
        .arg("--auto-zoom")
        .arg("--scale-down")
        .args(vec!["--geometry", "440x440"])
        .stdout(Stdio::piped())
        .output();

    //remove temp files/folder
    cleanup(Some(&args.dir), Some(downloaded_images.iter().map(|s| &*s).collect()));
}
