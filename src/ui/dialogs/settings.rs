use adw::prelude::*;

use crate::backend::spotify;
use super::sp_setup::sp_setup_dialog;

pub struct SettingsHandle {
    pub sp_row: adw::ActionRow,
    pub conn_btn: gtk::Button,
    pub disc_btn: gtk::Button,
}

pub fn settings(
    window: &adw::ApplicationWindow,
    dl_dir: &str,
    on_dir: impl Fn() + 'static,
    spotify_name: Option<&str>,
    on_spotify_connect: impl Fn() + 'static,
    on_spotify_disconnect: impl Fn() + 'static,
    on_close: impl Fn() + 'static,
) -> SettingsHandle {
    let d = adw::Window::builder()
        .title("Settings")
        .default_width(500)
        .default_height(400)
        .transient_for(window)
        .modal(true)
        .build();

    d.connect_close_request(move |_| {
        on_close();
        gtk::glib::Propagation::Proceed
    });

    let header = adw::HeaderBar::new();

    let dir_row = adw::ActionRow::builder()
        .title("Download folder")
        .subtitle(dl_dir)
        .build();

    let change_btn = gtk::Button::builder()
        .label("Change")
        .valign(gtk::Align::Center)
        .build();
    change_btn.connect_clicked(move |_| { on_dir(); });
    dir_row.add_suffix(&change_btn);

    let general_group = adw::PreferencesGroup::new();
    general_group.add(&dir_row);

    let client_id = spotify::load_client_id().unwrap_or_default();

    let id_row = adw::ActionRow::builder()
        .title("Client ID")
        .subtitle(if client_id.is_empty() { "not set" } else { &client_id })
        .build();

    let setup_btn = gtk::Button::builder()
        .label("Setup")
        .valign(gtk::Align::Center)
        .build();

    let win_ref = window.clone();
    let id_row_ref = id_row.clone();
    setup_btn.connect_clicked(move |_| {
        let row = id_row_ref.clone();
        sp_setup_dialog(&win_ref, move |new_id| {
            row.set_subtitle(&new_id);
        });
    });
    id_row.add_suffix(&setup_btn);

    let connected = spotify_name.is_some();

    let sp_row = adw::ActionRow::builder()
        .title("Account")
        .subtitle(spotify_name.unwrap_or("not connected"))
        .build();

    let connect_btn = gtk::Button::builder()
        .label("Connect")
        .valign(gtk::Align::Center)
        .visible(!connected)
        .build();
    connect_btn.add_css_class("suggested-action");
    let has_id = !client_id.is_empty();
    connect_btn.set_sensitive(has_id);
    if !has_id {
        connect_btn.set_tooltip_text(Some("set client id first"));
    }

    let disconnect_btn = gtk::Button::builder()
        .label("Disconnect")
        .valign(gtk::Align::Center)
        .visible(connected)
        .build();
    disconnect_btn.add_css_class("destructive-action");

    let sp_row_c = sp_row.clone();
    let disconnect_btn_c = disconnect_btn.clone();
    connect_btn.connect_clicked(move |b| {
        on_spotify_connect();
        sp_row_c.set_subtitle("connecting...");
        b.set_sensitive(false);
        disconnect_btn_c.set_visible(false);
    });

    let sp_row_d = sp_row.clone();
    let connect_btn_d = connect_btn.clone();
    disconnect_btn.connect_clicked(move |_| {
        on_spotify_disconnect();
        sp_row_d.set_subtitle("not connected");
        connect_btn_d.set_visible(true);
        connect_btn_d.set_sensitive(true);
    });

    sp_row.add_suffix(&connect_btn);
    sp_row.add_suffix(&disconnect_btn);

    let spotify_group = adw::PreferencesGroup::builder()
        .title("Spotify")
        .build();
    spotify_group.add(&id_row);
    spotify_group.add(&sp_row);

    let about_btn = gtk::Button::builder()
        .label("About")
        .halign(gtk::Align::Center)
        .margin_top(16)
        .margin_bottom(16)
        .build();
    about_btn.add_css_class("pill");
    about_btn.add_css_class("suggested-action");
    let d_ref = d.clone();
    about_btn.connect_clicked(move |_| { super::about::about(&d_ref); });

    let page = adw::PreferencesPage::new();
    page.add(&general_group);
    page.add(&spotify_group);

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    content.append(&page);
    content.append(&about_btn);

    let tb = adw::ToolbarView::new();
    tb.add_top_bar(&header);
    tb.set_content(Some(&content));
    d.set_content(Some(&tb));
    d.present();

    SettingsHandle {
        sp_row,
        conn_btn: connect_btn,
        disc_btn: disconnect_btn,
    }
}
