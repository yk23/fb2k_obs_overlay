use std::path::{Path, PathBuf};
use log::{error};

#[derive(Debug, Clone)]
pub struct MediaMetadata {
    pub song_id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f32,
    pub file_path: PathBuf,
    pub play_status: String,
}


impl MediaMetadata {
    pub(super) fn new(
        title: String,
        artist: String,
        album: String,
        duration: f32,
        file_path: PathBuf,
        play_status: String,
    ) -> Self {
        let song_info_to_hash = format!("{}", file_path.display());
        let song_id: String = format!("{:x}", md5::compute(song_info_to_hash));
        MediaMetadata {
            song_id, title, artist, album, duration, file_path, play_status,
        }
    }
}


struct AlbumArtError {
    reason: String
}


const ALBUM_ART_FILENAMES: &[&str] = &[
    "cover.jpg",
    "folder.jpg",
    "front.jpg",
    "cover.png",
    "folder.png",
    "front.png"
];


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


impl MediaMetadata {
    pub fn find_album_art_path(&self) -> Option<PathBuf> {
        let song_parent_dir = match find_album_art_candidate_dir(&self.file_path) {
            Ok(p) => p,
            Err(e) => {
                error!("Couldn't get album art directory. Reason: {}", e.reason);
                return None;
            }
        };

        for &candidate_fname in ALBUM_ART_FILENAMES.iter() {
            let candidate_path = song_parent_dir.join(candidate_fname);
            if candidate_path.exists() {
                return Some(candidate_path);
            }
        }

        error!("No album art found for {}", self.file_path.display());
        None
    }
}
