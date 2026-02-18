use std::path::PathBuf;
use std::time::Instant;

use adw::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

use crate::backend::spotify;
use crate::models::{Album, Artist, Track};
use super::dl_row::DlRow;
use super::handlers;
use super::result_row::{ResultItem, ResultRow, ResultRowOutput};

pub struct App {
    pub results: FactoryVecDeque<ResultRow>,
    pub downloads: FactoryVecDeque<DlRow>,
    pub dl_dir: PathBuf,
    pub searching: bool,
    pub busy: bool,
    pub status: String,
    pub search_entry: gtk::SearchEntry,
    pub filter: gtk::DropDown,
    pub logs: Vec<String>,
    pub next_dl_id: u64,
    pub eta: String,
    pub dl_started: Option<Instant>,
    pub dl_total: usize,
    pub dl_done: usize,
    pub sp_tokens: Option<spotify::Tokens>,
    pub sp_row: Option<adw::ActionRow>,
    pub sp_conn_btn: Option<gtk::Button>,
    pub sp_disc_btn: Option<gtk::Button>,
}

#[derive(Debug)]
pub enum Msg {
    CheckYtdlp,
    YtdlpMissing,
    YtdlpInstall(Result<(), String>),
    YtdlpReady,

    Search(String),
    SearchRes(Result<Vec<ResultItem>, String>),
    LoadArtist(Artist),
    ArtistAlbums(Result<Vec<Album>, String>),
    LoadAlbum(Album),
    AlbumTracks(Result<Vec<Track>, String>),
    SelectAll,
    DeselectAll,

    DlSelected,
    DlStart(Vec<Track>),
    DlProgress(u64, f64),
    DlDone(u64, Result<String, String>),

    SetDlDir,
    DlDirPicked(PathBuf),
    ShowLogs,
    ShowSettings,
    SettingsDone,

    SpConnect,
    SpAuth(Result<spotify::Tokens, String>),
    SpDisconnect,
    SpLibrary,
    SpLibRes(Result<Vec<spotify::Playlist>, String>),
    SpPlaylist(String, String),
    SpLiked,
    SpTracks(Result<Vec<Track>, String>),
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Music Downloader"),
            set_default_width: 900,
            set_default_height: 650,

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_title_widget: Some(&gtk::Label::new(Some("Music Downloader"))),
                    pack_end = &gtk::Button {
                        set_icon_name: "emblem-system-symbolic",
                        set_tooltip_text: Some("Settings"),
                        connect_clicked => Msg::ShowSettings,
                    },
                    pack_end = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_valign: gtk::Align::Center,

                        gtk::Label {
                            #[watch]
                            set_label: &model.eta,
                            #[watch]
                            set_visible: !model.eta.is_empty(),
                            add_css_class: "dim-label",
                        },
                        gtk::Button {
                            set_icon_name: "spotify-symbolic",
                            set_tooltip_text: Some("Spotify library"),
                            #[watch]
                            set_sensitive: model.sp_tokens.is_some(),
                            connect_clicked => Msg::SpLibrary,
                        },
                        gtk::Button {
                            set_icon_name: "utilities-terminal-symbolic",
                            set_tooltip_text: Some("View logs"),
                            connect_clicked => Msg::ShowLogs,
                        },
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        set_margin_all: 12,

                        model.filter.clone() {
                            set_valign: gtk::Align::Center,
                        },

                        model.search_entry.clone() {
                            set_hexpand: true,
                            set_placeholder_text: Some("search"),
                            connect_activate[sender] => move |entry| {
                                let q = entry.text().to_string();
                                if !q.is_empty() { sender.input(Msg::Search(q)); }
                            },
                        },

                        gtk::Button {
                            set_label: "Search",
                            add_css_class: "suggested-action",
                            #[watch]
                            set_sensitive: !model.searching,
                            connect_clicked[sender, entry = model.search_entry.clone()] => move |_| {
                                let q = entry.text().to_string();
                                if !q.is_empty() { sender.input(Msg::Search(q)); }
                            },
                        },
                    },

                    gtk::Separator {},

                    gtk::Paned {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_vexpand: true,
                        set_shrink_start_child: false,
                        set_shrink_end_child: false,
                        set_position: 500,

                        #[wrap(Some)]
                        set_start_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 8,
                                set_margin_all: 8,
                                gtk::Label {
                                    set_label: "Results",
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Start,
                                    add_css_class: "title-4",
                                },
                                gtk::Button { set_label: "All", add_css_class: "flat", connect_clicked => Msg::SelectAll },
                                gtk::Button { set_label: "None", add_css_class: "flat", connect_clicked => Msg::DeselectAll },
                                gtk::Button {
                                    set_icon_name: "document-save-symbolic",
                                    add_css_class: "suggested-action",
                                    connect_clicked => Msg::DlSelected,
                                },
                            },

                            gtk::Overlay {
                                set_vexpand: true,

                                #[wrap(Some)]
                                set_child = &gtk::ScrolledWindow {
                                    set_vexpand: true,
                                    set_hscrollbar_policy: gtk::PolicyType::Never,
                                    #[local_ref]
                                    result_list -> gtk::ListBox {
                                        set_selection_mode: gtk::SelectionMode::None,
                                        set_activate_on_single_click: false,
                                        add_css_class: "boxed-list",
                                        set_margin_start: 8,
                                        set_margin_end: 8,
                                        set_margin_bottom: 8,
                                    },
                                },

                                add_overlay = &gtk::Spinner {
                                    set_halign: gtk::Align::Center,
                                    set_valign: gtk::Align::Center,
                                    set_width_request: 48,
                                    set_height_request: 48,
                                    #[watch]
                                    set_spinning: model.busy,
                                    #[watch]
                                    set_visible: model.busy,
                                },
                            },
                        },

                        #[wrap(Some)]
                        set_end_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            gtk::Box {
                                set_margin_all: 8,
                                gtk::Label {
                                    set_label: "Downloads",
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Start,
                                    add_css_class: "title-4",
                                },
                            },

                            gtk::ScrolledWindow {
                                set_vexpand: true,
                                set_hscrollbar_policy: gtk::PolicyType::Never,
                                #[local_ref]
                                download_list -> gtk::ListBox {
                                    set_selection_mode: gtk::SelectionMode::None,
                                    add_css_class: "boxed-list",
                                    set_margin_start: 8,
                                    set_margin_end: 8,
                                    set_margin_bottom: 8,
                                },
                            },
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        set_margin_all: 8,
                        #[watch]
                        set_visible: !model.status.is_empty(),

                        gtk::Spinner {
                            #[watch]
                            set_spinning: model.busy,
                            #[watch]
                            set_visible: model.busy,
                        },

                        gtk::Label {
                            #[watch]
                            set_label: &model.status,
                            set_halign: gtk::Align::Start,
                            add_css_class: "dim-label",
                        },
                    },
                },
            },
        }
    }

    fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        load_resources();

        let display = gtk::prelude::WidgetExt::display(&root);
        gtk::IconTheme::for_display(&display).add_resource_path("/org/musicdownloader/icons");

        let results = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |out| match out {
                ResultRowOutput::Artist(a) => Msg::LoadArtist(a),
                ResultRowOutput::Album(a) => Msg::LoadAlbum(a),
                ResultRowOutput::SpotifyPlaylist(id, name) => Msg::SpPlaylist(id, name),
                ResultRowOutput::SpotifyLiked => Msg::SpLiked,
            });

        let downloads = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |_: ()| unreachable!());

        let filter = gtk::DropDown::from_strings(&["All", "Albums", "Artists", "Tracks"]);
        filter.set_selected(0);

        let model = App {
            results,
            downloads,
            dl_dir: crate::config::dl_dir(),
            searching: false,
            busy: false,
            status: String::new(),
            search_entry: gtk::SearchEntry::new(),
            filter,
            logs: Vec::new(),
            next_dl_id: 0,
            eta: String::new(),
            dl_started: None,
            dl_total: 0,
            dl_done: 0,
            sp_tokens: spotify::load_tokens(),
            sp_row: None,
            sp_conn_btn: None,
            sp_disc_btn: None,
        };

        let result_list = model.results.widget();
        let download_list = model.downloads.widget();
        let widgets = view_output!();

        sender.input(Msg::CheckYtdlp);
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        handlers::handle(self, msg, sender, root);
    }
}

fn load_resources() {
    let bytes = gtk::glib::Bytes::from_static(
        include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource")),
    );
    let resource = gtk::gio::Resource::from_data(&bytes).expect("failed to load resources");
    gtk::gio::resources_register(&resource);
}
