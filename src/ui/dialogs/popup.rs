use adw::prelude::*;

pub fn show_popup(window: &adw::ApplicationWindow, icon: &str, title: &str, subtitle: &str) {
    let d = adw::Window::builder()
        .title("")
        .default_width(360)
        .default_height(280)
        .transient_for(window)
        .modal(true)
        .resizable(false)
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .spacing(8)
        .build();

    let img = gtk::Image::builder().icon_name(icon).pixel_size(80).margin_top(32).build();
    img.add_css_class("dim-label");
    content.append(&img);

    let t = gtk::Label::new(Some(title));
    t.add_css_class("title-1");
    t.set_margin_top(8);
    content.append(&t);

    let s = gtk::Label::new(Some(subtitle));
    s.add_css_class("dim-label");
    content.append(&s);

    let close = gtk::Button::builder()
        .label("Close")
        .halign(gtk::Align::Center)
        .margin_top(16)
        .margin_bottom(32)
        .build();
    close.add_css_class("suggested-action");
    close.add_css_class("pill");
    let d_ref = d.clone();
    close.connect_clicked(move |_| d_ref.close());
    content.append(&close);

    d.set_content(Some(&content));
    d.present();
}
