use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use log::{info, warn};
use rustnao::Sauce;
use serde_json::Value;
use std::os::unix::prelude::ExitStatusExt;
use std::borrow::Borrow;

///Downloads one image through curl.
///
/// arguments:
/// url: String - the url to download the image from
/// name: String - what to title the file
/// dir: PathBuf - where to download the file to
///
/// returns:
/// ExitStatus - the result of the curl command
fn download_image(url: String, name: String, dir: PathBuf) -> ExitStatus {
    info!("Downloading '{}' as '{}'.", url, name);
    if let Ok(out) = Command::new("/usr/bin/curl")
        .args(vec!["-X", "GET"])
        .arg(url)
        .args(vec!["-H", "accept: image/*"])
        .args(vec!["-o".to_string(), name])
        .current_dir(dir)
        .stdout(Stdio::piped())
        .output() {
        out.status
    } else {
        warn!("downloading failed!");
        ExitStatus::from_raw(2)
    }
}

/// Downloads a vector of thumbnails, given a vector of Sauces
///
/// arguments:
/// result: Vec<Sauce> - a vector of sauces
/// dir: PathBuf - path to download images to
///
/// returns:
/// A vector of PathBufs, indicating the downloaded thumbnails
pub fn download_images(result: Vec<Sauce>, dir: PathBuf) -> Vec<PathBuf> {
    info!("Downloading at most {} image(s).", result.len());
    let mut i: u16 = 0;
    let mut out: Vec<PathBuf> = Vec::new();
    let null: Value = Value::from("null");

    for res in result {
        if let Some(fields) = res.additional_fields {
            let char = fields.get("characters").unwrap_or(null.borrow()).as_str().unwrap();
            let prop = fields.get("material").unwrap_or(null.borrow()).as_str().unwrap();
            let name = format!("{}: {}_{}", prop, char, i);

            if download_image(res.thumbnail, name.clone(), dir.clone()).success() {
                out.push(dir.join(name));
            }
            i = i + 1;
        }
    }
    out
}
