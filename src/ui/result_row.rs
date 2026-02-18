use adw::prelude::*;
use relm4::prelude::*;

use crate::backend::spotify;
use crate::models::{Album, Artist, Track};

#[derive(Debug, Clone)]
pub enum ResultItem {
    Artist(Artist),
    Album(Album),
    Track(Track),
    SpotifyPlaylist(spotify::Playlist),
    SpotifyLiked,
}

pub struct ResultRow {
    pub item: ResultItem,
    pub selected: bool,
}

#[derive(Debug)]
pub enum ResultRowMsg {
    Toggle,
    Browse,
}

#[derive(Debug)]
pub enum ResultRowOutput {
    Album(Album),
    Artist(Artist),
    SpotifyPlaylist(String, String),
    SpotifyLiked,
}

#[relm4::factory(pub)]
impl FactoryComponent for ResultRow {
    type Init = ResultItem;
    type Input = ResultRowMsg;
    type Output = ResultRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 12,
            set_margin_all: 8,

            gtk::CheckButton {
                #[watch]
                set_active: self.selected,
                #[watch]
                set_visible: matches!(self.item, ResultItem::Album(_) | ResultItem::Track(_)),
                connect_toggled => ResultRowMsg::Toggle,
            },

            gtk::Image {
                set_icon_name: Some(match &self.item {
                    ResultItem::Artist(_) => "avatar-default-symbolic",
                    ResultItem::Album(_) => "media-optical-cd-audio-symbolic",
                    ResultItem::Track(_) => "audio-x-generic-symbolic",
                    ResultItem::SpotifyPlaylist(_) => "view-list-bullet-symbolic",
                    ResultItem::SpotifyLiked => "starred-symbolic",
                }),
                set_pixel_size: 32,
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 4,
                set_hexpand: true,

                gtk::Label {
                    set_label: &match &self.item {
                        ResultItem::Artist(a) => a.name.clone(),
                        ResultItem::Album(a) => a.title.clone(),
                        ResultItem::Track(t) => match t.track_pos {
                            Some(n) => format!("{n}. {}", t.title),
                            None => t.title.clone(),
                        },
                        ResultItem::SpotifyPlaylist(p) => p.name.clone(),
                        ResultItem::SpotifyLiked => String::from("Liked Songs"),
                    },
                    set_halign: gtk::Align::Start,
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                    add_css_class: "heading",
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    gtk::Label {
                        set_label: &match &self.item {
                            ResultItem::Artist(a) => format!("{} albums", a.nb_album),
                            ResultItem::Album(a) => a.artist.clone(),
                            ResultItem::Track(t) => t.artist.clone(),
                            ResultItem::SpotifyPlaylist(p) => format!("{} tracks", p.nb_tracks),
                            ResultItem::SpotifyLiked => String::from("spotify"),
                        },
                        set_halign: gtk::Align::Start,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        add_css_class: "dim-label",
                    },
                    gtk::Label {
                        set_label: "·",
                        add_css_class: "dim-label",
                        #[watch]
                        set_visible: match &self.item {
                            ResultItem::Album(a) => a.nb_tracks > 0,
                            ResultItem::Track(_) => true,
                            _ => false,
                        },
                    },
                    gtk::Label {
                        set_label: &match &self.item {
                            ResultItem::Album(a) if a.nb_tracks > 0 => format!("{} tracks", a.nb_tracks),
                            ResultItem::Track(t) => t.duration_fmt(),
                            _ => String::new(),
                        },
                        add_css_class: "dim-label",
                        #[watch]
                        set_visible: match &self.item {
                            ResultItem::Album(a) => a.nb_tracks > 0,
                            ResultItem::Track(_) => true,
                            _ => false,
                        },
                    },
                },
            },

            gtk::Button {
                set_icon_name: "go-next-symbolic",
                add_css_class: "flat",
                set_tooltip_text: Some("browse"),
                #[watch]
                set_visible: matches!(self.item,
                    ResultItem::Album(_) | ResultItem::Artist(_)
                    | ResultItem::SpotifyPlaylist(_) | ResultItem::SpotifyLiked
                ),
                connect_clicked => ResultRowMsg::Browse,
            },
        }
    }

    fn init_model(item: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { item, selected: false }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            ResultRowMsg::Toggle => {
                if matches!(self.item, ResultItem::Album(_) | ResultItem::Track(_)) {
                    self.selected = !self.selected;
                }
            }
            ResultRowMsg::Browse => match &self.item {
                ResultItem::Album(a) => { let _ = sender.output(ResultRowOutput::Album(a.clone())); }
                ResultItem::Artist(a) => { let _ = sender.output(ResultRowOutput::Artist(a.clone())); }
                ResultItem::SpotifyPlaylist(p) => {
                    let _ = sender.output(ResultRowOutput::SpotifyPlaylist(p.id.clone(), p.name.clone()));
                }
                ResultItem::SpotifyLiked => { let _ = sender.output(ResultRowOutput::SpotifyLiked); }
                _ => {}
            },
        }
    }
}
