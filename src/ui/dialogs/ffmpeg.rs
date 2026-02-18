use adw::prelude::*;

pub fn ffmpeg_missing(window: &adw::ApplicationWindow) {
    let d = adw::MessageDialog::new(
        Some(window),
        Some("ffmpeg not found"),
        Some("ffmpeg is needed for audio conversion. install it with your package manager."),
    );
    d.add_response("ok", "OK");
    d.present();
}
