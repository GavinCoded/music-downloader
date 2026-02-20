use std::fs;
use serde::Deserialize;
use tokio::process::Command;
use crate::config;

const RELEASE: &str = "https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest";

#[derive(Deserialize)]
struct Release { tag_name: String, assets: Vec<Asset> }

#[derive(Deserialize)]
struct Asset { name: String, browser_download_url: String }

pub async fn check() -> bool {
    Command::new(config::ytdlp_bin())
        .arg("--version")
        .output().await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub async fn install() -> Result<(), String> {
    let client = reqwest::Client::builder()
        .user_agent("music-downloader")
        .build().map_err(|e| format!("client: {e}"))?;

    let rel: Release = client.get(RELEASE)
        .send().await.map_err(|e| format!("req: {e}"))?
        .json().await.map_err(|e| format!("parse: {e}"))?;

    let name = if cfg!(target_os = "windows") { "yt-dlp.exe" } else { "yt-dlp_linux" };
    let asset = rel.assets.iter().find(|a| a.name == name)
        .ok_or_else(|| format!("{name} not in release"))?;

    let bytes = client.get(&asset.browser_download_url)
        .send().await.map_err(|e| format!("dl: {e}"))?
        .bytes().await.map_err(|e| format!("read: {e}"))?;

    let path = config::ytdlp_path();
    fs::create_dir_all(config::data_dir()).map_err(|e| format!("mkdir: {e}"))?;
    fs::write(&path, &bytes).map_err(|e| format!("write: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("chmod: {e}"))?;
    }

    Ok(())
}

pub async fn chk_update_ytdlp() -> Result<Option<String>, String> {
    let out = Command::new(config::ytdlp_bin())
        .arg("--version")
        .output().await
        .map_err(|e| format!("version: {e}"))?;
    let local = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if local.is_empty() {
        return Err("could not read local version".into());
    }

    let client = reqwest::Client::builder()
        .user_agent("music-downloader")
        .build().map_err(|e| format!("client: {e}"))?;

    let rel: Release = client.get(RELEASE)
        .send().await.map_err(|e| format!("req: {e}"))?
        .json().await.map_err(|e| format!("parse: {e}"))?;

    let remote = rel.tag_name.trim().to_string();
    if remote != local {
        Ok(Some(remote))
    } else {
        Ok(None)
    }
}
