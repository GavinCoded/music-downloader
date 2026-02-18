use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub track_pos: Option<u32>,
    pub cover_url: String,
    pub is_album_track: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DlStatus {
    Queued,
    Active(f64),
    Done,
    Failed(String),
}

impl Track {
    pub fn duration_fmt(&self) -> String {
        let m = self.duration as u64 / 60;
        let s = self.duration as u64 % 60;
        format!("{m}:{s:02}")
    }

    pub fn from_dz(dt: &DzTrack, album_fb: &str, cover_fb: &str) -> Self {
        let cover = dt
            .album
            .as_ref()
            .map(|a| a.cover_xl.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| cover_fb.to_string());

        Self {
            title: dt.title.clone(),
            artist: dt.artist.as_ref().map_or(String::new(), |a| a.name.clone()),
            album: dt.album.as_ref().map_or(album_fb.to_string(), |a| a.title.clone()),
            duration: dt.duration,
            track_pos: dt.track_position,
            cover_url: cover,
            is_album_track: false,
        }
    }

    pub fn yt_query(&self) -> String {
        format!("{} - {}", self.artist, self.title)
    }
}

#[derive(Debug, Deserialize)]
pub struct DzTrackRes {
    #[serde(default)]
    pub data: Vec<DzTrack>,
}

#[derive(Debug, Deserialize)]
pub struct DzTrack {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub duration: f64,
    #[serde(default)]
    pub track_position: Option<u32>,
    #[serde(default)]
    pub artist: Option<DzArtist>,
    #[serde(default)]
    pub album: Option<DzAlbumRef>,
}

#[derive(Debug, Deserialize)]
pub struct DzArtist {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DzAlbumRef {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub cover_xl: String,
}
