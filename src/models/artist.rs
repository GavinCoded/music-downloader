use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub nb_album: u32,
}

#[derive(Debug, Deserialize)]
pub struct DzArtistRes {
    #[serde(default)]
    pub data: Vec<DzArtist>,
}

#[derive(Debug, Deserialize)]
pub struct DzArtist {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nb_album: u32,
}

impl Artist {
    pub fn from_dz(da: &DzArtist) -> Self {
        Self {
            id: da.id,
            name: da.name.clone(),
            nb_album: da.nb_album,
        }
    }
}
