use std::{
    fs,
    path::{Path, PathBuf}
};
use serde::Deserialize;
use anyhow::Result;
use super::metadata::MediaMetadata;


#[derive(Deserialize)]
struct JSONMediaMetadata {
    title: String,
    artist: String,
    album: String,
    duration: f32,
    file_path: PathBuf,
    play_status: String,
}



impl From<JSONMediaMetadata> for MediaMetadata {
    fn from(json_md: JSONMediaMetadata) -> Self {
        MediaMetadata::new(
            json_md.title,
            json_md.artist,
            json_md.album,
            json_md.duration,
            json_md.file_path,
            json_md.play_status,
        )
    }
}


pub fn deserialize_json_file(json_path: &Path) -> Result<MediaMetadata> {
    let json_str = fs::read_to_string(json_path)?;
    // Strip UTF-8 BOM if present
    let json_str = json_str.trim_start_matches('\u{FEFF}');

    let metadata = serde_json::from_str::<JSONMediaMetadata>(&json_str)?;
    Ok(metadata.into())
}
