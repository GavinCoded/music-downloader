use adw::prelude::*;

pub fn ytdlp_outdated(window: &adw::ApplicationWindow, version: &str, on_update: impl Fn() + 'static) {
    let d = adw::MessageDialog::new(
        Some(window),
        Some("yt-dlp update available"),
        Some(&format!("a new version of yt-dlp is available ({version}). update now?")),
    );
    d.add_response("skip", "Skip");
    d.add_response("update", "Update");
    d.set_response_appearance("update", adw::ResponseAppearance::Suggested);
    d.set_close_response("skip");
    d.connect_response(None, move |_, r| { if r == "update" { on_update(); } });
    d.present();
}
