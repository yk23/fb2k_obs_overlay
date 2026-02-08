use askama::Template;
use crate::media_info::metadata::MediaMetadata;

#[derive(Template)]
#[template(path = "metadata.json", escape = "none")]
pub struct MetadataJSONTemplate<'a> {
    pub status: &'a str,
    pub song_id: &'a str,
    pub title: &'a str,
    pub artist: &'a str,
    pub album: &'a str,
    pub duration: u32,
}


impl<'a> From<&'a MediaMetadata> for MetadataJSONTemplate<'a> {
    fn from(metadata: &'a MediaMetadata) -> Self {
        Self {
            status: &metadata.play_status,
            song_id: &metadata.song_id,
            title: &metadata.title,
            artist: &metadata.artist,
            album: &metadata.album,
            duration: metadata.duration as u32,
        }
    }
}
