use adw::prelude::*;
use std::path::PathBuf;

pub fn pick_folder(window: &adw::ApplicationWindow, on_chosen: impl Fn(PathBuf) + 'static) {
    let d = gtk::FileChooserDialog::new(
        Some("Select download folder"),
        Some(window),
        gtk::FileChooserAction::SelectFolder,
        &[("Cancel", gtk::ResponseType::Cancel), ("Select", gtk::ResponseType::Accept)],
    );
    d.connect_response(move |dlg, r| {
        if r == gtk::ResponseType::Accept {
            if let Some(p) = dlg.file().and_then(|f| f.path()) {
                on_chosen(p);
            }
        }
        dlg.close();
    });
    d.present();
}
