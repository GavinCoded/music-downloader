mod backend;
mod config;
mod models;
mod ui;

use relm4::prelude::*;
use ui::App;

fn main() {
    let app = RelmApp::new("com.musicdownloader.app");
    app.run::<App>(());
}
