use adw::prelude::*;

pub fn log_viewer(window: &adw::ApplicationWindow, logs: &str) {
    let d = adw::Window::builder()
        .title("Download Logs")
        .default_width(700)
        .default_height(500)
        .transient_for(window)
        .modal(true)
        .build();

    let view = gtk::TextView::builder()
        .editable(false)
        .monospace(true)
        .wrap_mode(gtk::WrapMode::WordChar)
        .top_margin(8)
        .bottom_margin(8)
        .left_margin(8)
        .right_margin(8)
        .build();

    view.buffer().set_text(logs);

    let scroll = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .child(&view)
        .build();

    let header = adw::HeaderBar::new();
    let tb = adw::ToolbarView::new();
    tb.add_top_bar(&header);
    tb.set_content(Some(&scroll));
    d.set_content(Some(&tb));
    d.present();
}
