use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::config;
use crate::models::Track;
use super::deezer;

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub async fn download<F>(track: &Track, base: &Path, on_progress: F) -> Result<String, String>
where
    F: Fn(f64) + Send + 'static,
{
    let dir = track_dir(base, track);
    let query = format!("ytsearch1:{}", track.yt_query());
    let tpl = dir.join("%(title)s.%(ext)s").to_string_lossy().to_string();

    let mut child = Command::new(config::ytdlp_bin())
        .args([
            "-x", "--audio-format", "mp3", "--audio-quality", "0",
            "--no-embed-metadata", "--no-embed-thumbnail",
            "--no-warnings", "--no-playlist",
            "--newline", "--progress",
            "--print", "after_move:filepath",
            "-o", &tpl,
        ])
        .arg(&query)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn().map_err(|e| format!("spawn: {e}"))?;

    let mut mp3_path = None;
    let mut log = String::new();

    if let Some(stdout) = child.stdout.take() {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            log.push_str(&line);
            log.push('\n');
            if let Some(pct) = parse_pct(&line) { on_progress(pct); }
            let trimmed = line.trim();
            if trimmed.ends_with(".mp3") && Path::new(trimmed).exists() {
                mp3_path = Some(PathBuf::from(trimmed));
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            log.push_str("[stderr] ");
            log.push_str(&line);
            log.push('\n');
        }
    }

    let status = child.wait().await.map_err(|e| format!("wait: {e}"))?;
    if !status.success() { return Err(format!("yt-dlp failed\n{log}")); }

    let mp3 = mp3_path.ok_or_else(|| format!("mp3 not found\n{log}"))?;
    on_progress(90.0);

    let cover = fetch_cover_tmp(track).await;
    embed_meta(&mp3, track, cover.as_deref()).await?;

    let final_path = dir.join(format!("{} - {}.mp3", track.artist, track.title));
    if final_path != mp3 { let _ = fs::rename(&mp3, &final_path); }

    on_progress(100.0);
    Ok(log)
}

fn track_dir(base: &Path, track: &Track) -> PathBuf {
    if track.is_album_track && !track.album.is_empty() && !track.artist.is_empty() {
        let dir = base.join(format!("{} - {}", track.artist, track.album));
        let _ = fs::create_dir_all(&dir);
        dir
    } else {
        base.to_path_buf()
    }
}

async fn embed_meta(mp3: &Path, track: &Track, cover: Option<&Path>) -> Result<(), String> {
    let tmp = mp3.with_extension("tmp.mp3");

    let mut args: Vec<String> = vec![
        "-y".into(), "-i".into(), mp3.to_string_lossy().into(),
    ];

    if let Some(c) = cover {
        args.extend(["-i".into(), c.to_string_lossy().into()]);
        args.extend(["-map".into(), "0:a".into(), "-map".into(), "1:0".into()]);
    } else {
        args.extend(["-map".into(), "0:a".into()]);
    }

    args.extend(["-c".into(), "copy".into(), "-id3v2_version".into(), "3".into()]);
    args.extend(["-metadata".into(), format!("title={}", track.title)]);
    args.extend(["-metadata".into(), format!("artist={}", track.artist)]);
    args.extend(["-metadata".into(), format!("album={}", track.album)]);
    if let Some(pos) = track.track_pos {
        args.extend(["-metadata".into(), format!("track={pos}")]);
    }
    if cover.is_some() {
        args.extend(["-metadata:s:v".into(), "title=Album cover".into()]);
        args.extend(["-metadata:s:v".into(), "comment=Cover (front)".into()]);
    }
    args.push(tmp.to_string_lossy().into());

    let out = Command::new("ffmpeg").args(&args).output().await
        .map_err(|e| format!("ffmpeg: {e}"))?;

    if let Some(c) = cover { let _ = fs::remove_file(c); }

    if !out.status.success() {
        let _ = fs::remove_file(&tmp);
        return Err(format!("ffmpeg err: {}", String::from_utf8_lossy(&out.stderr)));
    }

    fs::rename(&tmp, mp3).map_err(|e| format!("rename: {e}"))
}

async fn fetch_cover_tmp(track: &Track) -> Option<PathBuf> {
    if track.cover_url.is_empty() { return None; }
    let data = deezer::fetch_cover(&track.cover_url).await.ok()?;
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("mdl_cover_{}_{n}.jpg", std::process::id()));
    fs::write(&path, &data).ok()?;
    Some(path)
}

fn parse_pct(line: &str) -> Option<f64> {
    if !line.contains("[download]") { return None; }
    line.split_whitespace()
        .find(|s| s.ends_with('%'))?
        .trim_end_matches('%')
        .parse().ok()
}
