use std::path::{PathBuf};

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
