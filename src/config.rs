use std::path::PathBuf;

pub fn dl_dir() -> PathBuf {
    dirs::audio_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join("Music"))
}

pub fn data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".local/share"))
        .join("music-downloader")
}

pub fn ytdlp_path() -> PathBuf {
    data_dir().join("yt-dlp")
}

pub fn ytdlp_bin() -> String {
    let path = ytdlp_path();
    if path.exists() {
        path.to_string_lossy().to_string()
    } else {
        String::from("yt-dlp")
    }
}

pub fn spotify_tokens_path() -> PathBuf {
    data_dir().join("spotify_tokens.json")
}
