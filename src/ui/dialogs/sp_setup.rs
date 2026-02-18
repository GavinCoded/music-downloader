use adw::prelude::*;

use crate::backend::spotify;

pub fn sp_setup_dialog(window: &adw::ApplicationWindow, on_saved: impl Fn(String) + 'static) {
    let d = adw::Window::builder()
        .title("Spotify Setup")
        .default_width(500)
        .default_height(400)
        .transient_for(window)
        .modal(true)
        .build();

    let header = adw::HeaderBar::new();

    let instructions = gtk::Label::builder()
        .label(
            "1. go to developer.spotify.com/dashboard\n\
             2. create an app (any name)\n\
             3. set redirect uri to:\n   http://127.0.0.1:18492/callback\n\
             4. copy the client id and paste it below"
        )
        .wrap(true)
        .halign(gtk::Align::Start)
        .margin_start(16)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .build();

    let entry = gtk::Entry::builder()
        .placeholder_text("paste client id here")
        .margin_start(16)
        .margin_end(16)
        .build();

    if let Some(existing) = spotify::load_client_id() {
        entry.set_text(&existing);
    }

    let save_btn = gtk::Button::builder()
        .label("Save")
        .halign(gtk::Align::End)
        .margin_start(16)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .build();
    save_btn.add_css_class("suggested-action");

    let d_ref = d.clone();
    let entry_ref = entry.clone();
    save_btn.connect_clicked(move |_| {
        let id = entry_ref.text().trim().to_string();
        if !id.is_empty() {
            spotify::save_client_id(&id);
            on_saved(id);
        }
        d_ref.close();
    });

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    content.append(&instructions);
    content.append(&entry);
    content.append(&save_btn);

    let tb = adw::ToolbarView::new();
    tb.add_top_bar(&header);
    tb.set_content(Some(&content));
    d.set_content(Some(&tb));
    d.present();
}
