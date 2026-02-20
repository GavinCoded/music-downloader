use relm4::prelude::*;

use crate::backend;
use crate::models::{DlStatus, Track};
use super::app::{App, Msg};
use super::dialogs;
use super::result_row::ResultItem;

pub fn dl_selected(app: &mut App, sender: ComponentSender<App>) {
    let guard = app.results.guard();
    let mut tracks = Vec::new();
    let mut albums = Vec::new();
    for i in 0..guard.len() {
        let Some(r) = guard.get(i) else { continue };
        if !r.selected { continue; }
        match &r.item {
            ResultItem::Track(t) => tracks.push(t.clone()),
            ResultItem::Album(a) => albums.push(a.clone()),
            _ => {}
        }
    }
    drop(guard);

    if tracks.is_empty() && albums.is_empty() {
        app.status = String::from("none selected");
        return;
    }

    app.busy = true;
    if !albums.is_empty() {
        app.status = format!("fetching {} albums", albums.len());
        let s = sender.input_sender().clone();
        relm4::spawn(async move {
            let mut all = tracks;
            for album in &albums {
                if let Ok(t) = backend::deezer::album_tracks(album).await {
                    all.extend(t);
                }
            }
            s.emit(Msg::DlStart(all));
        });
    } else {
        sender.input(Msg::DlStart(tracks));
    }
}

pub fn dl_tracks(app: &mut App, tracks: Vec<Track>, sender: ComponentSender<App>) {
    if tracks.is_empty() {
        app.status = String::from("no tracks found");
        return;
    }

    app.status = format!("dl {} tracks", tracks.len());
    app.dl_started = Some(std::time::Instant::now());
    app.dl_total = tracks.len();
    app.dl_done = 0;

    let mut ids = Vec::new();
    let mut guard = app.downloads.guard();
    for track in &tracks {
        let id = app.next_dl_id;
        app.next_dl_id += 1;
        ids.push(id);
        guard.push_front((id, track.clone()));
    }
    drop(guard);

    let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(3));
    for (track, id) in tracks.into_iter().zip(ids) {
        let s = sender.input_sender().clone();
        let dir = app.dl_dir.clone();
        let sem = sem.clone();
        relm4::spawn(async move {
            let _permit = sem.acquire().await;
            let ps = s.clone();
            let result = backend::ytdlp::download(&track, &dir, move |pct| {
                ps.emit(Msg::DlProgress(id, pct));
            })
            .await;
            s.emit(Msg::DlDone(id, result));
        });
    }
}

pub fn dl_progress(app: &mut App, id: u64, pct: f64) {
    let mut guard = app.downloads.guard();
    for i in 0..guard.len() {
        if guard.get(i).map_or(false, |r| r.id == id) {
            if let Some(row) = guard.get_mut(i) {
                row.progress = pct;
                row.status = DlStatus::Active(pct);
            }
            break;
        }
    }
    drop(guard);
    update_eta(app);
}

pub fn dl_done(app: &mut App, id: u64, result: Result<String, String>) {
    let mut guard = app.downloads.guard();
    let mut log_entry = None;
    for i in 0..guard.len() {
        if guard.get(i).map_or(false, |r| r.id == id) {
            if let Some(row) = guard.get_mut(i) {
                let label = format!("{} - {}", row.track.artist, row.track.title);
                match &result {
                    Ok(log) => {
                        row.progress = 100.0;
                        row.status = DlStatus::Done;
                        log_entry = Some(format!("=== {label} ===\n{log}"));
                    }
                    Err(e) => {
                        row.status = DlStatus::Failed(e.clone());
                        log_entry = Some(format!("=== fail: {label} ===\n{e}"));
                    }
                }
            }
            break;
        }
    }
    let total = guard.len();
    let done = (0..total)
        .filter(|i| {
            guard.get(*i).map_or(false, |d| {
                matches!(d.status, DlStatus::Done | DlStatus::Failed(_))
            })
        })
        .count();
    drop(guard);

    if let Some(entry) = log_entry {
        if let Some(handle) = &app.log_handle {
            dialogs::append_log(handle, &entry);
        }
        app.logs.push(entry);
    }
    app.dl_done = done;
    if done == total {
        app.busy = false;
        app.status = format!("done ({total} tracks)");
        app.eta = String::new();
        app.dl_started = None;
    } else {
        app.status = format!("{done}/{total}");
        update_eta(app);
    }
}

fn update_eta(app: &mut App) {
    let started = match app.dl_started {
        Some(t) => t,
        None => {
            app.eta = String::new();
            return;
        }
    };
    let remaining = app.dl_total.saturating_sub(app.dl_done);
    if remaining == 0 || app.dl_done == 0 {
        app.eta = String::new();
        return;
    }
    let elapsed = started.elapsed().as_secs_f64();
    let per_track = elapsed / app.dl_done as f64;
    let secs_left = (per_track * remaining as f64) as u64;
    let m = secs_left / 60;
    let s = secs_left % 60;
    app.eta = format!("eta {m:02}:{s:02}");
}
