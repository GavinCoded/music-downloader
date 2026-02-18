use relm4::prelude::*;

use crate::backend;
use super::app::{App, Msg};
use super::dialogs;
use super::dl;
use super::search;
use super::sp;

pub fn handle(app: &mut App, msg: Msg, sender: ComponentSender<App>, root: &adw::ApplicationWindow) {
    match msg {
        Msg::CheckDeps => {
            app.status = String::from("chk: deps");
            let s = sender.input_sender().clone();
            relm4::spawn(async move {
                if !backend::ffmpeg::check().await {
                    s.emit(Msg::FfmpegMissing);
                }
                s.emit(if backend::ytdlp_setup::check().await {
                    Msg::YtdlpReady
                } else {
                    Msg::YtdlpMissing
                });
            });
        }
        Msg::FfmpegMissing => {
            app.status = String::from("ffmpeg missing");
            dialogs::ffmpeg_missing(root);
        }
        Msg::YtdlpMissing => {
            app.status = String::from("yt-dlp missing");
            let s = sender.input_sender().clone();
            dialogs::ytdlp_missing(root, move || {
                let sc = s.clone();
                relm4::spawn(async move {
                    sc.emit(Msg::YtdlpInstall(backend::ytdlp_setup::install().await));
                });
            });
        }
        Msg::YtdlpInstall(Ok(())) => app.status = String::from("yt-dlp ok"),
        Msg::YtdlpInstall(Err(e)) => {
            app.status = format!("install err: {e}");
            dialogs::ytdlp_install_failed(root, &e);
        }
        Msg::YtdlpReady => {
            if app.status == "chk: yt-dlp" { app.status = String::from("ready"); }
        }

        Msg::Search(q) => search::search(app, q, sender),
        Msg::SearchRes(Ok(items)) => search::search_done(app, items),
        Msg::SearchRes(Err(e)) => { app.searching = false; app.busy = false; app.status = e; }
        Msg::LoadArtist(a) => search::browse_artist(app, a, sender),
        Msg::ArtistAlbums(Ok(albums)) => search::artist_albums(app, albums),
        Msg::ArtistAlbums(Err(e)) => { app.busy = false; app.status = format!("err: {e}"); }
        Msg::LoadAlbum(a) => search::browse_album(app, a, sender),
        Msg::AlbumTracks(Ok(tracks)) => search::album_tracks(app, tracks),
        Msg::AlbumTracks(Err(e)) => { app.busy = false; app.status = format!("err: {e}"); }
        Msg::SelectAll => search::select_all(app, true),
        Msg::DeselectAll => search::select_all(app, false),

        Msg::DlSelected => dl::dl_selected(app, sender),
        Msg::DlStart(tracks) => dl::dl_tracks(app, tracks, sender),
        Msg::DlProgress(id, pct) => dl::dl_progress(app, id, pct),
        Msg::DlDone(id, result) => dl::dl_done(app, id, result),

        Msg::SpConnect => sp::connect(app, sender),
        Msg::SpAuth(Ok(tokens)) => sp::auth_done(app, tokens, root),
        Msg::SpAuth(Err(e)) => { app.busy = false; app.status = format!("spotify err: {e}"); }
        Msg::SpDisconnect => sp::disconnect(app, root),
        Msg::SpLibrary => sp::load_library(app, sender),
        Msg::SpLibRes(Ok(playlists)) => sp::library_loaded(app, playlists),
        Msg::SpLibRes(Err(e)) => { app.busy = false; app.status = format!("spotify err: {e}"); }
        Msg::SpPlaylist(id, name) => sp::load_playlist(app, id, name, sender),
        Msg::SpLiked => sp::load_liked(app, sender),
        Msg::SpTracks(Ok(tracks)) => sp::tracks_loaded(app, tracks),
        Msg::SpTracks(Err(e)) => { app.busy = false; app.status = format!("spotify err: {e}"); }

        Msg::SetDlDir => {
            let s = sender.input_sender().clone();
            dialogs::pick_folder(root, move |p| { s.emit(Msg::DlDirPicked(p)); });
        }
        Msg::DlDirPicked(p) => app.dl_dir = p,
        Msg::ShowLogs => {
            let text = if app.logs.is_empty() { String::from("no logs yet") } else { app.logs.join("\n\n") };
            dialogs::log_viewer(root, &text);
        }
        Msg::SettingsDone => {
            app.sp_row = None;
            app.sp_conn_btn = None;
            app.sp_disc_btn = None;
        }
        Msg::ShowSettings => {
            let name = app.sp_tokens.as_ref().map(|t| t.display_name.clone());
            let s = sender.input_sender();
            let (s1, s2, s3, s4) = (s.clone(), s.clone(), s.clone(), s.clone());
            let handle = dialogs::settings(
                root,
                &app.dl_dir.display().to_string(),
                move || s1.emit(Msg::SetDlDir),
                name.as_deref(),
                move || s2.emit(Msg::SpConnect),
                move || s3.emit(Msg::SpDisconnect),
                move || s4.emit(Msg::SettingsDone),
            );
            app.sp_row = Some(handle.sp_row);
            app.sp_conn_btn = Some(handle.conn_btn);
            app.sp_disc_btn = Some(handle.disc_btn);
        }
    }
}
