use relm4::{
    adw::{
        prelude::*,
    }, 
    gtk::{
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib,
    }, prelude::*
};

use crate::credits::CreditsModel;

#[derive(Debug)]
pub enum DialogMsg {
    Show,
    OpenCredits,
}

#[derive(Debug)]
pub struct AwesomeModel {
    window: Option<adw::Dialog>,
    navigation: adw::NavigationView,
    credits_model: Controller<CreditsModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for AwesomeModel {
    type Init = ();
    type Input = DialogMsg;
    type Output = ();

    view! {
        adw::Dialog {
            // set_title: &gettext("List of used e-imzo websites"),
            // set_follows_content_size: true,
            set_presentation_mode: adw::DialogPresentationMode::Floating,
            set_content_width: 360,

            #[wrap(Some)]
            #[name = "navigation"]
            set_child = &adw::NavigationView {
                add = &adw::NavigationPage {
                    set_title: "Search",

                    adw::ToolbarView {
                        set_top_bar_style: adw::ToolbarStyle::Flat,

                        add_top_bar = &adw::HeaderBar {
                            #[wrap(Some)]
                            set_title_widget = &adw::WindowTitle {
                                set_title: "Search"
                            }
                        },

                        #[wrap(Some)]
                        set_content = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            // set_spacing: 12,
                            set_margin_horizontal: 24,
                            
                            gtk::Image {
                                set_vexpand: true,
                                set_hexpand: true,
                                set_paintable: Some(&embedded_logo()),
                                set_pixel_size: 125,
                            },

                            gtk::Label {
                                set_label: "Fractal",
                                add_css_class: "title-1",
                                set_margin_top: 15,
                            },

                            gtk::Label {
                                set_label: "Fraktal jamoasi",
                                set_margin_top: 10,
                            },

                            gtk::Button {
                                set_label: "13",
                                set_margin_top: 8,
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

                            // PreferencesPage || PreferencesGroup => EntryRow => ActionRow
                            gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_margin_top: 30,
                                set_selection_mode: gtk::SelectionMode::None,
                                
                                adw::ActionRow {
                                    set_title: "Vebsayt",
                                    add_suffix = &gtk::LinkButton {
                                        set_uri: "https://gitlab.gnome.org/World/fractal/",
                                        #[wrap(Some)]
                                        set_child = &gtk::Image {
                                            set_icon_name: Some("external-link-symbolic"),
                                            inline_css: "color: white;",
                                        }
                                    },  
                                },
                            },

                            gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_margin_top: 18,
                                set_selection_mode: gtk::SelectionMode::None,
                                
                                adw::ActionRow {
                                    set_title: "Yordam uchuns savollar",
                                    add_suffix = &gtk::LinkButton {
                                        set_uri: "",
                                        #[wrap(Some)]
                                        set_child = &gtk::Image {
                                            set_icon_name: Some("external-link-symbolic"),
                                            inline_css: "color: white",
                                        }
                                    }
                                },

                                adw::ActionRow {
                                    set_title: "Muammo haqida xabar berish",
                                    add_suffix = &gtk::LinkButton {
                                        set_uri: "",
                                        #[wrap(Some)]
                                        set_child = &gtk::Image {
                                            set_icon_name: Some("external-link-symbolic"),
                                            inline_css: "color: white",
                                        }
                                    }
                                }
                            },

                            gtk::ListBox {
                                set_selection_mode: gtk::SelectionMode::None,
                                add_css_class: "boxed-list",
                                set_margin_top: 18,
                                set_margin_bottom: 20,
                                
                                adw::ActionRow {
                                    set_title: "Kreditlar",
                                    add_suffix = &gtk::LinkButton {
                                        set_uri: "",
                                        connect_clicked => DialogMsg::OpenCredits,
                                        #[wrap(Some)]
                                        set_child = &gtk::Image {
                                            set_icon_name: Some("go-next-symbolic"),
                                            inline_css: "color: white",
                                        }
                                    }
                                },

                                adw::ActionRow {
                                    set_title: "Huquqiy",
                                    add_suffix = &gtk::LinkButton {
                                        set_uri: "https://gitlab.gnome.org/World/fractal/-/issues",
                                        #[wrap(Some)]
                                        set_child = &gtk::Image {
                                            set_icon_name: Some("go-next-symbolic"),
                                            inline_css: "color: white",
                                        }
                                    }
                                }
                            },
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
        let widgets = view_output!();
        let credits_model = CreditsModel::builder().launch(()).detach();

        let model = AwesomeModel {
            window: Some(root.clone()),
            credits_model: credits_model,
            navigation: widgets.navigation.clone()
        };
        

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show => {
                if let Some(dialog) = &self.window {
                    let parent = relm4::main_application().active_window();
                    dialog.present(parent.as_ref());
                }
            },
            DialogMsg::OpenCredits => {
                let page = self.credits_model.widget();
                self.navigation.push(page);
            }
        }
    }
}

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("../assets/logo.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}