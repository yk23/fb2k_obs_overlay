use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};
use log::error;
use id3::Tag;
use imageformat::{
    detect_image_format, ImageFormat as IMGImageFormat
};


struct AlbumArtError {
    reason: String
}


impl From<Error> for AlbumArtError {
    fn from(err: Error) -> Self {
        Self { reason: format!("Encountered IO Error: {}", err) }
    }
}


pub struct AlbumArt {
    pub format: ImageFormat,
    pub image: Vec<u8>
}

impl AlbumArt {
    fn create_from_data(image_data: Vec<u8>) -> Result<Self, AlbumArtError> {
        let mut image_cursor = std::io::Cursor::new(&image_data);
        let img_fmt = detect_image_format(&mut image_cursor)?;
        let return_fmt = match img_fmt {
            IMGImageFormat::Png => ImageFormat::PNG,
            IMGImageFormat::Jpeg => ImageFormat::JPEG,
            IMGImageFormat::Webp => ImageFormat::WEBP,
            IMGImageFormat::Gif => ImageFormat::GIF,
            _ => return Err(AlbumArtError{ reason: format!("Encountered unsupported image format: {}", img_fmt) })
        };

        Ok(Self {
            format: return_fmt,
            image: image_data,
        })
    }
}


pub enum ImageFormat {
    PNG, JPEG, GIF, WEBP
}


const EXTERNAL_ALBUM_ART_FILENAMES: &[&str] = &[
    "cover.jpg",
    "folder.jpg",
    "front.jpg",
    "cover.png",
    "folder.png",
    "front.png"
];


/// Helper function for function `external_album_art`. Returns the parent directory of the
/// requested music file, bypassing "CD*" dirs.
fn find_album_art_candidate_dir(song_file_path: &Path) -> Result<PathBuf, AlbumArtError> {
    let song_parent_dir = song_file_path.parent()
        .ok_or_else(|| AlbumArtError { reason: format!("Directory {:?} has no parent", song_file_path) })?;
    let parent_dir_name = song_parent_dir.file_name()
        .ok_or_else(|| AlbumArtError { reason: format!("Directory {:?} has no name", song_parent_dir) })?
        .to_str()
        .ok_or_else(|| AlbumArtError { reason: format!("Couldn't convert parent dir name from {:?}", song_parent_dir) })?;

    if parent_dir_name.starts_with("CD") {
        let cd_parent_dir = song_parent_dir.parent()
            .ok_or_else(|| AlbumArtError { reason: format!("Directory {:?} has no parent", song_parent_dir) })?;
        Ok(cd_parent_dir.to_path_buf())
    } else {
        Ok(song_parent_dir.to_path_buf())
    }
}

///
/// Searches for any external (meaning: non-embedded) candidate album art file (e.g. front.jpg) for the song file.
fn external_album_art_file(song_path: &Path) -> Option<PathBuf> {
    let song_parent_dir = match find_album_art_candidate_dir(song_path) {
        Ok(p) => p,
        Err(e) => {
            // This shouldn't happen, but errors like this shouldn't crash the program. Gracefully log the error.
            error!("Couldn't search for album art directory. Reason: {}", e.reason);
            return None;
        }
    };

    for &candidate_fname in EXTERNAL_ALBUM_ART_FILENAMES.iter() {
        let candidate_path = song_parent_dir.join(candidate_fname);
        if candidate_path.exists() {
            return Some(candidate_path);
        }
    }

    // no external album art file found (there may still be internal embedded ones)
    None
}


pub fn external_album_art(song_path: &Path) -> Option<AlbumArt> {
    let img_path = external_album_art_file(song_path)?;
    let image_data = fs::read(&img_path)
        .map_err(|e| error!("Image read error: {}", e))
        .ok()?;
    AlbumArt::create_from_data(image_data)
        .map_err(|e| error!("Couldn't pass album art data into struct. Reason: {}", e.reason))
        .ok()
}


pub fn embedded_album_art(song_path: &Path) -> Option<AlbumArt> {
    let tag = Tag::read_from_path(song_path)
        .map_err(|e| error!("Couldn't read id3 tag from file {}. Reason: {}", song_path.display(), e))
        .ok()?;

    let picture = tag.pictures().next()?;
    AlbumArt::create_from_data(picture.data.clone())
        .map_err(|e| error!("Couldn't pass album art data into struct. Reason: {}", e.reason))
        .ok()
}


