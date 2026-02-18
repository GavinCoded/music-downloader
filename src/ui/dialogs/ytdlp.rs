use adw::prelude::*;

pub fn ytdlp_missing(window: &adw::ApplicationWindow, on_install: impl Fn() + 'static) {
    let d = adw::MessageDialog::new(
        Some(window),
        Some("yt-dlp not found"),
        Some("yt-dlp is needed for downloads. install it now?"),
    );
    d.add_response("cancel", "Cancel");
    d.add_response("install", "Install");
    d.set_response_appearance("install", adw::ResponseAppearance::Suggested);
    d.set_close_response("cancel");
    d.connect_response(None, move |_, r| { if r == "install" { on_install(); } });
    d.present();
}

pub fn ytdlp_install_failed(window: &adw::ApplicationWindow, err: &str) {
    let d = adw::MessageDialog::new(
        Some(window),
        Some("install failed"),
        Some(&format!("yt-dlp install failed: {err}")),
    );
    d.add_response("ok", "OK");
    d.present();
}
