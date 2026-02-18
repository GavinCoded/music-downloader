use relm4::prelude::*;

use crate::backend;
use super::app::{App, Msg};
use super::result_row::ResultItem;

pub fn search(app: &mut App, query: String, sender: ComponentSender<App>) {
    app.searching = true;
    app.busy = true;
    app.status = format!("searching \"{query}\"");
    app.results.guard().clear();
    let selected = app.filter.selected();
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        let mut items = Vec::new();
        if selected == 0 || selected == 1 {
            if let Ok(albums) = backend::deezer::search_albums(&query).await {
                items.extend(albums.into_iter().take(10).map(ResultItem::Album));
            }
        }
        if selected == 0 || selected == 2 {
            if let Ok(artists) = backend::deezer::search_artists(&query).await {
                items.extend(artists.into_iter().take(5).map(ResultItem::Artist));
            }
        }
        if selected == 0 || selected == 3 {
            if let Ok(tracks) = backend::deezer::search_tracks(&query).await {
                items.extend(tracks.into_iter().map(ResultItem::Track));
            }
        }
        s.emit(if items.is_empty() {
            Msg::SearchRes(Err(String::from("no results")))
        } else {
            Msg::SearchRes(Ok(items))
        });
    });
}

pub fn search_done(app: &mut App, items: Vec<ResultItem>) {
    app.searching = false;
    app.busy = false;
    app.status = format!("{} results", items.len());
    let mut guard = app.results.guard();
    for item in items {
        guard.push_back(item);
    }
}

pub fn browse_artist(app: &mut App, artist: crate::models::Artist, sender: ComponentSender<App>) {
    app.busy = true;
    app.status = format!("loading \"{}\"", artist.name);
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        s.emit(Msg::ArtistAlbums(backend::deezer::artist_albums(&artist).await));
    });
}

pub fn artist_albums(app: &mut App, albums: Vec<crate::models::Album>) {
    app.busy = false;
    app.status = format!("{} albums", albums.len());
    let mut guard = app.results.guard();
    guard.clear();
    for album in albums {
        guard.push_back(ResultItem::Album(album));
    }
}

pub fn browse_album(app: &mut App, album: crate::models::Album, sender: ComponentSender<App>) {
    app.busy = true;
    app.status = format!("loading \"{}\"", album.title);
    let s = sender.input_sender().clone();
    relm4::spawn(async move {
        s.emit(Msg::AlbumTracks(backend::deezer::album_tracks(&album).await));
    });
}

pub fn album_tracks(app: &mut App, tracks: Vec<crate::models::Track>) {
    app.busy = false;
    app.status = format!("{} tracks", tracks.len());
    let mut guard = app.results.guard();
    guard.clear();
    for track in tracks {
        guard.push_back(ResultItem::Track(track));
    }
}

pub fn select_all(app: &mut App, active: bool) {
    use adw::prelude::*;
    let len = app.results.guard().len();
    let list = app.results.widget();
    for i in 0..len {
        let cb = list.row_at_index(i as i32)
            .and_then(|r| r.child())
            .and_then(|c| c.first_child())
            .and_then(|c| c.downcast::<gtk::CheckButton>().ok());
        if let Some(cb) = cb.filter(|c| c.is_visible()) {
            cb.set_active(active);
        }
    }
}
