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
    dialog: Controller<Dialog>,
}

struct App {
    header: Controller<Header>,
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

                gtk::Image {
                    set_vexpand: true,
                    set_hexpand: true,
                    set_paintable: Some(&embedded_logo()),
                    set_pixel_size: 125
                },

                gtk::Label {
                    set_label: "Fractal",
                    add_css_class: css::classes::TITLE_1,
                    set_margin_top: 10,
                },

                gtk::Label {
                    set_label: "Fraktal jamoasi",
                    set_margin_top: 10,
                },

                gtk::Button {
                    set_label: "13",
                    connect_clicked: move |_| {
                        
                    },
                    add_css_class: "accent",
                    add_css_class: "pill",
                    set_halign: gtk::Align::Center,
                    set_margin_top: 8,
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

                gtk::LinkButton {
                    set_label: "hi"
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

        let model = App { header };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {
        
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
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AwesomeModel;

#[relm4::component(pub)]
impl SimpleComponent for AwesomeModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::Dialog {
            set_title: &gettext("List of used e-imzo websites"),
            set_follows_content_size: true,
            set_presentation_mode: adw::DialogPresentationMode::Floating,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,

                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: "my.gov.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://my.gov.uz/uz")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "ahost.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://clients.ahost.uz/login")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "id.egov.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://id.egov.uz/oz")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "didox.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://didox.uz/login_with_signature")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "birdarcha.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://new.birdarcha.uz/login")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "e-invoice.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://e-invoice.uz/register/")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },

                        adw::ActionRow {
                            set_title: "my.mehnat.uz",
                            add_suffix = &gtk::LinkButton::builder()
                                .uri("https://my.mehnat.uz/login#")
                                .child(&gtk::Image::from_icon_name("document-send-symbolic"))
                                .build(),
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();
        let window = relm4::main_application().active_window();
        root.present(window.as_ref());

        ComponentParts { model, widgets }
    }
}