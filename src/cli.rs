use std::path::PathBuf;
use log::{debug, error};
use std::ffi::OsString;
use std::process::exit;
use std::fs;
use clap::Parser;

/// Determine what characters/properties are depicted in an image.
///
/// Makes use of the saucenao API to fetch thumbnails matching a given image.
/// These thumbnails are then opened in feh to allow the user to confirm if there is a match or not.
/// All thumbnails are automatically removed on exit.
#[derive(Parser)]
pub struct Cli {
    /// Path to the image to be sourced.
    pub file: PathBuf,
    /// Folder to download thumbnails to.
    #[arg(short = 'd', long = "directory", default_value = "/tmp/img_search/")]
    pub dir: PathBuf,
}

/// This function checks if a given file is an image
///
/// arguments:
/// path: &PathBuf - path to the file to check
///
/// returns:
/// boolean indicating if file is an image or not
fn is_image(path: &PathBuf) -> bool {
    let ext = path.extension();
    match ext {
        Some(ext) => {
            vec![OsString::from("png"),
                 OsString::from("jpg"),
                 OsString::from("jpeg"),
                 OsString::from("webp")]
                .contains(&ext.to_ascii_lowercase())
        }
        None => false
    }
}

/// This function checks if a given file exists
///
/// arguments:
/// path: &PathBuf - path to the file to check
///
/// returns:
/// boolean indicating if the file exists or not
fn file_exists(path: &PathBuf) -> bool {
    match path.try_exists() {
        Ok(bool) => bool,
        Err(e) => {
            error!("{}", e.to_string());
            false
        }
    }
}

/// This function checks if the given flags are valid input
///
/// Arguments:
/// args: Cli - the command line flags to be checked
pub fn check_input(args: &mut Cli) {
    //fixme: rebase this whole thing
    if !file_exists(&args.file) {
        error!("Input file \"{}\" does not exist!", args.file.display());
        exit(1);
    }

    if !is_image(&args.file) {
        error!("Input file \"{}\" is not an image!", args.file.display());
        exit(1);
    }

    if args.file.is_relative() {
        debug!("Converting relative input file to absolute.");
        match args.file.canonicalize() {
            Ok(p) => args.file = p,
            Err(e) => {
                error!("Failed to canonicalize file: {}.", e.to_string());
                exit(1);
            }
        }
    }

    if file_exists(&args.dir) && args.dir.is_file() {
        error!("Input directory exists as a file!");
        exit(1);
    }

    if args.dir.is_dir() {
        error!("Input directory \"{}\" already exists!", args.dir.display());
        exit(1);
    } else {
        if let Err(e) =  fs::create_dir_all(&args.dir) {
            error!("Failed to create directory \"{}\": {}", args.dir.display(), e.to_string());
            exit(1);
        }
    }

    if args.dir.is_relative() {
        debug!("Converting relative input directory to absolute.");
        match args.dir.canonicalize() {
            Ok(p) => args.dir = p,
            Err(e) => {
                error!("Failed to canonicalize directory: {}.", e.to_string());
                crate::cleanup(Some(&args.dir), None);
                exit(1);
            }
        }
    }
}
