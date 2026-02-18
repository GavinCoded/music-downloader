use adw::prelude::*;
use relm4::prelude::*;

use crate::models::{DlStatus, Track};

pub struct DlRow {
    pub id: u64,
    pub track: Track,
    pub status: DlStatus,
    pub progress: f64,
}

#[derive(Debug)]
pub enum DlRowMsg {}

#[relm4::factory(pub)]
impl FactoryComponent for DlRow {
    type Init = (u64, Track);
    type Input = DlRowMsg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,
            set_margin_all: 8,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 4,
                set_hexpand: true,

                gtk::Label {
                    set_label: &self.track.title,
                    set_halign: gtk::Align::Start,
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                    add_css_class: "heading",
                },
                gtk::Label {
                    set_label: &self.track.artist,
                    set_halign: gtk::Align::Start,
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                    add_css_class: "dim-label",
                },
                gtk::ProgressBar {
                    #[watch]
                    set_fraction: self.progress / 100.0,
                    #[watch]
                    set_visible: matches!(self.status, DlStatus::Active(_)),
                    add_css_class: "osd",
                },
            },

            gtk::Label {
                #[watch]
                set_label: &match &self.status {
                    DlStatus::Queued => String::from("queued"),
                    DlStatus::Active(p) => format!("{p:.0}%"),
                    DlStatus::Done => String::from("done"),
                    DlStatus::Failed(e) => format!("fail: {e}"),
                },
                #[watch]
                add_css_class: match &self.status {
                    DlStatus::Done => "success",
                    DlStatus::Failed(_) => "error",
                    DlStatus::Active(_) => "accent",
                    _ => "dim-label",
                },
                set_halign: gtk::Align::End,
                set_valign: gtk::Align::Center,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            id: init.0,
            track: init.1,
            status: DlStatus::Queued,
            progress: 0.0,
        }
    }

    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}
