use super::types::Tokens;

pub fn save_tokens(tokens: &Tokens) {
    let path = crate::config::spotify_tokens_path();
    let _ = std::fs::create_dir_all(path.parent().unwrap_or(&path));
    let _ = std::fs::write(&path, serde_json::to_string(tokens).unwrap_or_default());
}

pub fn load_tokens() -> Option<Tokens> {
    let data = std::fs::read_to_string(crate::config::spotify_tokens_path()).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn clear_tokens() {
    let _ = std::fs::remove_file(crate::config::spotify_tokens_path());
}

pub fn save_client_id(id: &str) {
    let path = crate::config::data_dir().join("spotify_client_id");
    let _ = std::fs::create_dir_all(path.parent().unwrap_or(&path));
    let _ = std::fs::write(&path, id);
}

pub fn load_client_id() -> Option<String> {
    let s = std::fs::read_to_string(crate::config::data_dir().join("spotify_client_id")).ok()?;
    let s = s.trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}
