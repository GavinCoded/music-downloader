use crate::models::{Album, Artist, Track};
use crate::models::album::DzAlbumRes;
use crate::models::artist::DzArtistRes;
use crate::models::track::DzTrackRes;

const API: &str = "https://api.deezer.com";
const LIMIT: u32 = 25;

async fn get<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    reqwest::get(url).await
        .map_err(|e| format!("req: {e}"))?
        .json().await
        .map_err(|e| format!("parse: {e}"))
}

pub async fn search_artists(q: &str) -> Result<Vec<Artist>, String> {
    let res: DzArtistRes = get(&format!("{API}/search/artist?q={}&limit={LIMIT}", enc(q))).await?;
    Ok(res.data.iter().map(Artist::from_dz).collect())
}

pub async fn search_albums(q: &str) -> Result<Vec<Album>, String> {
    let res: DzAlbumRes = get(&format!("{API}/search/album?q={}&limit={LIMIT}", enc(q))).await?;
    Ok(res.data.iter().map(Album::from_dz).collect())
}

pub async fn search_tracks(q: &str) -> Result<Vec<Track>, String> {
    let res: DzTrackRes = get(&format!("{API}/search/track?q={}&limit={LIMIT}", enc(q))).await?;
    Ok(res.data.iter().map(|dt| Track::from_dz(dt, "", "")).collect())
}

pub async fn artist_albums(artist: &Artist) -> Result<Vec<Album>, String> {
    let res: DzAlbumRes = get(&format!("{API}/artist/{}/albums?limit=100", artist.id)).await?;
    Ok(res.data.iter().map(Album::from_dz).collect())
}

pub async fn album_tracks(album: &Album) -> Result<Vec<Track>, String> {
    let res: DzTrackRes = get(&format!("{API}/album/{}/tracks?limit=100", album.id)).await?;
    Ok(res.data.iter().map(|dt| {
        let mut t = Track::from_dz(dt, &album.title, &album.cover_url);
        t.is_album_track = true;
        t
    }).collect())
}

pub async fn fetch_cover(url: &str) -> Result<Vec<u8>, String> {
    if url.is_empty() { return Err(String::from("no cover url")); }
    Ok(reqwest::get(url).await
        .map_err(|e| format!("cover: {e}"))?
        .bytes().await
        .map_err(|e| format!("read: {e}"))?
        .to_vec())
}

fn enc(s: &str) -> String {
    s.chars().map(|c| match c {
        ' ' => '+'.to_string(),
        c if c.is_alphanumeric() || "-_.~".contains(c) => c.to_string(),
        c => format!("%{:02X}", c as u32),
    }).collect()
}
