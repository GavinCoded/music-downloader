use adw::prelude::*;

fn link_btn(label: &str, url: &'static str) -> gtk::Button {
    let btn = gtk::Button::builder()
        .label(label)
        .halign(gtk::Align::Center)
        .build();
    btn.add_css_class("flat");
    btn.add_css_class("accent");
    btn.connect_clicked(move |_| {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    });
    btn
}

pub fn about(parent: &adw::Window) {
    let d = adw::Window::builder()
        .title("About")
        .default_width(360)
        .default_height(400)
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .spacing(4)
        .build();

    let icon = gtk::Image::builder()
        .icon_name("audio-x-generic-symbolic")
        .pixel_size(80)
        .margin_top(32)
        .build();
    icon.add_css_class("dim-label");
    content.append(&icon);

    let title = gtk::Label::new(Some("Music Downloader"));
    title.add_css_class("title-1");
    title.set_margin_top(16);
    content.append(&title);

    let ver = gtk::Label::new(Some("alpha v1.0.0"));
    ver.add_css_class("dim-label");
    content.append(&ver);

    let sep = gtk::Separator::builder()
        .margin_top(16)
        .margin_bottom(4)
        .margin_start(40)
        .margin_end(40)
        .build();
    content.append(&sep);

    let author_lbl = gtk::Label::new(Some("Author"));
    author_lbl.add_css_class("title-4");
    author_lbl.set_margin_top(4);
    content.append(&author_lbl);
    content.append(&link_btn("GavinStrikes", "https://github.com/GavinCoded"));

    let contrib_lbl = gtk::Label::new(Some("Contributors"));
    contrib_lbl.add_css_class("title-4");
    contrib_lbl.set_margin_top(8);
    content.append(&contrib_lbl);
    content.append(&link_btn("sagevk", "https://github.com/sagevk"));

    let close = gtk::Button::builder()
        .label("Close")
        .halign(gtk::Align::Center)
        .margin_top(16)
        .margin_bottom(24)
        .build();
    close.add_css_class("pill");
    close.add_css_class("suggested-action");
    let d_ref = d.clone();
    close.connect_clicked(move |_| d_ref.close());
    content.append(&close);

    d.set_content(Some(&content));
    d.present();
}
