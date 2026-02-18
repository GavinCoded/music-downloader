use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub nb_tracks: u32,
}

#[derive(Debug, Deserialize)]
pub struct DzAlbumRes {
    #[serde(default)]
    pub data: Vec<DzAlbum>,
}

#[derive(Debug, Deserialize)]
pub struct DzAlbum {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub nb_tracks: u32,
    #[serde(default)]
    pub cover_xl: String,
    #[serde(default)]
    pub artist: Option<DzAlbumArtist>,
}

#[derive(Debug, Deserialize)]
pub struct DzAlbumArtist {
    #[serde(default)]
    pub name: String,
}

impl Album {
    pub fn from_dz(da: &DzAlbum) -> Self {
        Self {
            id: da.id,
            title: da.title.clone(),
            artist: da.artist.as_ref().map_or(String::new(), |a| a.name.clone()),
            cover_url: da.cover_xl.clone(),
            nb_tracks: da.nb_tracks,
        }
    }
}
