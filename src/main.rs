use std::convert::identity;

use relm4::{
    adw,
    css, gtk::{
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib,
        prelude::*,
    }, prelude::*
};

struct Header;

#[relm4::component]
impl SimpleComponent for Header {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: relm4::css::LINKED,

                gtk::Label {
                    set_label: ""
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

#[derive(Debug)]
enum AppMsg {
    AwesomeModel, Close
}


#[derive()]
struct App {
    header: Controller<Header>,
    dialog: Controller<AwesomeModel>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        gtk::ApplicationWindow
        {
            set_default_size: (500, 250),
            set_titlebar: Some(model.header.widget()),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::LinkButton {
                    set_label: "hi",
                    connect_clicked => AppMsg::AwesomeModel,
                },
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = Header::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let dialog = AwesomeModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| AppMsg::Close);
        

        let model = App { header, dialog };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AwesomeModel => {
                self.dialog.emit(DialogMsg::Show);
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
        }   
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.components");
    app.run::<App>(());
}

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("../assets/logo.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}


use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::gtk;

#[derive(Debug)]
pub enum DialogMsg {
    Show,
}

#[derive(Debug, Clone)]
pub struct AwesomeModel {
    window: Option<adw::Dialog>,
}

#[relm4::component(pub)]
impl SimpleComponent for AwesomeModel {
    type Init = ();
    type Input = DialogMsg;
    type Output = ();

    view! {
        adw::Dialog {
            set_title: &gettext("List of used e-imzo websites"),
            set_follows_content_size: true,
            set_presentation_mode: adw::DialogPresentationMode::Floating,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 24,
                    
                    gtk::Image {
                        set_vexpand: true,
                        set_hexpand: true,
                        set_paintable: Some(&embedded_logo()),
                        set_pixel_size: 125,
                    },

                    gtk::Label {
                        set_label: "Fractal",
                        add_css_class: "title-1",
                    },

                    gtk::Label {
                        set_label: "Fraktal jamoasi",
                    },

                    gtk::Button {
                        set_label: "13",
                        add_css_class: "accent",
                        add_css_class: "pill",
                        set_halign: gtk::Align::Center,
                        inline_css: "
                            padding: 6px 20px; 
                            min-height: 0px; 
                            min-width: 0px;
                            label { 
                                margin: 0px; 
                                padding: 0px; 
                            }
                        ",
                    },

                    gtk::ListBox {
                        add_css_class: "boxed-list",
                        set_margin_top: 20,

                        adw::ActionRow {
                            set_title: "Veb sayt",
                            // set_activatable: true,
                            // connect_activated => |_| {

                            // },
                            // set_subtitle: "Yagona interaktiv davlat xizmatlari portali",
                            
                            add_suffix = &gtk::LinkButton {
                                set_uri: "https://my.gov.uz/uz",
                                #[wrap(Some)]
                                set_child = &gtk::Image {
                                    set_icon_name: Some("external-link-symbolic"),
                                    inline_css: "color: white;",
                                }
                            },  
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AwesomeModel {
            window: Some(root.clone()), 
        };
        
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show => {
                if let Some(dialog) = &self.window {
                    let parent = relm4::main_application().active_window();
                    dialog.present(parent.as_ref());
                }
            }
        }
    }
}