use adw::prelude::*;
use gtk::prelude::TextBufferExt;

pub struct LogHandle {
    pub buf: gtk::TextBuffer,
    pub scroll: gtk::ScrolledWindow,
}

pub fn log_viewer(window: &adw::ApplicationWindow, logs: &str) -> LogHandle {
    let d = adw::Window::builder()
        .title("Download Logs")
        .default_width(700)
        .default_height(500)
        .transient_for(window)
        .modal(false)
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

    let buf = view.buffer();
    buf.set_text(logs);

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

    scroll_to_bottom(&scroll);

    LogHandle { buf, scroll }
}

pub fn append_log(handle: &LogHandle, text: &str) {
    let buf = &handle.buf;
    let mut end = buf.end_iter();
    let prefix = if buf.char_count() > 0 { "\n\n" } else { "" };
    buf.insert(&mut end, &format!("{prefix}{text}"));
    scroll_to_bottom(&handle.scroll);
}

fn scroll_to_bottom(scroll: &gtk::ScrolledWindow) {
    let sw = scroll.clone();
    gtk::glib::idle_add_local_once(move || {
        let adj = sw.vadjustment();
        adj.set_value(adj.upper() - adj.page_size());
    });
}
