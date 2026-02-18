use adw::prelude::*;
use relm4::prelude::*;

use crate::backend::spotify;
use super::app::{App, Msg};
use super::dialogs;
use super::result_row::ResultItem;

pub fn connect(app: &mut App, sender: ComponentSender<App>) {
    let client_id = match spotify::load_client_id() {
        Some(id) => id,
        None => {
            app.status = String::from("set client id in settings first");
            return;
        }
    };
    app.busy = true;
    app.status = String::from("waiting for spotify auth...");
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        s.emit(Msg::SpAuth(spotify::authorize(&client_id).await));
    });
}

pub fn auth_done(app: &mut App, tokens: spotify::Tokens, root: &adw::ApplicationWindow) {
    app.busy = false;
    let name = tokens.display_name.clone();
    app.status = format!("connected as {name}");
    spotify::save_tokens(&tokens);
    app.sp_tokens = Some(tokens);
    if let Some(row) = &app.sp_row {
        row.set_subtitle(&name);
    }
    if let Some(btn) = &app.sp_conn_btn {
        btn.set_visible(false);
    }
    if let Some(btn) = &app.sp_disc_btn {
        btn.set_visible(true);
    }
    dialogs::show_popup(root, "object-select-symbolic", "Account Connected", &format!("signed in as {name}"));
}

pub fn disconnect(app: &mut App, root: &adw::ApplicationWindow) {
    spotify::clear_tokens();
    app.sp_tokens = None;
    app.status = String::from("spotify disconnected");
    if let Some(row) = &app.sp_row {
        row.set_subtitle("not connected");
    }
    if let Some(btn) = &app.sp_conn_btn {
        btn.set_visible(true);
        btn.set_sensitive(true);
    }
    if let Some(btn) = &app.sp_disc_btn {
        btn.set_visible(false);
    }
    dialogs::show_popup(root, "object-select-symbolic", "Account Disconnected", "spotify account has been disconnected");
}

pub fn load_library(app: &mut App, sender: ComponentSender<App>) {
    let tokens = match &app.sp_tokens {
        Some(t) => t.clone(),
        None => {
            app.status = String::from("not connected");
            return;
        }
    };
    app.busy = true;
    app.status = String::from("loading spotify library...");
    app.results.guard().clear();
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        s.emit(Msg::SpLibRes(spotify::playlists(&tokens).await));
    });
}

pub fn library_loaded(app: &mut App, playlists: Vec<spotify::Playlist>) {
    app.busy = false;
    app.status = format!("{} playlists", playlists.len() + 1);
    let mut guard = app.results.guard();
    guard.clear();
    guard.push_back(ResultItem::SpotifyLiked);
    for p in playlists {
        guard.push_back(ResultItem::SpotifyPlaylist(p));
    }
}

pub fn load_playlist(app: &mut App, id: String, name: String, sender: ComponentSender<App>) {
    let tokens = match &app.sp_tokens {
        Some(t) => t.clone(),
        None => return,
    };
    app.busy = true;
    app.status = format!("loading \"{name}\"");
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        s.emit(Msg::SpTracks(spotify::playlist_tracks(&tokens, &id).await));
    });
}

pub fn load_liked(app: &mut App, sender: ComponentSender<App>) {
    let tokens = match &app.sp_tokens {
        Some(t) => t.clone(),
        None => return,
    };
    app.busy = true;
    app.status = String::from("loading liked songs...");
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        s.emit(Msg::SpTracks(spotify::liked_tracks(&tokens).await));
    });
}

pub fn tracks_loaded(app: &mut App, tracks: Vec<crate::models::Track>) {
    app.busy = false;
    app.status = format!("{} tracks", tracks.len());
    let mut guard = app.results.guard();
    guard.clear();
    for t in tracks {
        guard.push_back(ResultItem::Track(t));
    }
}
