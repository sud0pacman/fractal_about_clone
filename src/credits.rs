use relm4::{
    adw::{
        self,
        prelude::*
    },
    gtk,
    prelude::*
};

#[derive(Debug)]
pub struct CreditsModel;

#[relm4::component(pub)]
impl SimpleComponent for CreditsModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: "Kreditlar",

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,
                set_width_request: 360,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Search Locations"
                    }
                },


                gtk::Box {
                    set_halign: gtk::Align::Center,
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Box {
                        set_margin_top: 10,
                        set_margin_start: 15,
                        set_margin_end: 15,
                        set_hexpand: true,

                        gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "Filesystem locations which are selected by system apps, such as Files",
                            add_css_class: "grey_color",
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Default Locations",
                        set_margin_top: 10,

                        adw::ActionRow {
                            set_title: "Home",
                            // set_hexpand: true,
                            set_subtitle: "Subfolders must be manually added for this location",
                            // set_activatable: true,

                            add_suffix = &gtk::Switch {
                                set_active: true,
                                set_valign: gtk::Align::Center,
                            },
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Custom Locations",
                        // set_halign: gtk::Align::Center,

                        adw::ActionRow {
                            set_title: "Desktop",
                            set_subtitle: "Location not found",
                            set_activatable: true,

                            add_suffix = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_hexpand: true,
                                set_halign: gtk::Align::End,

                                gtk::Button {
                                    set_icon_name: "document-open",
                                    add_css_class: "flat",
                                    set_valign: gtk::Align::Center,
                                },

                                gtk::Button {
                                    set_icon_name: "edit-delete",
                                    add_css_class: "flat",
                                    set_valign: gtk::Align::Center,
                                }
                            }
                        },

                        adw::ActionRow {
                            set_title: "Add Location",
                            set_activatable: true,
                            set_halign: gtk::Align::Center,

                            add_css_class: "bold",

                            add_prefix = &gtk::Image {
                                set_icon_name: Some("list-add-symbolic"),
                                set_valign: gtk::Align::Center,
                            }
                        },
                    }
                }
            }
        }
    }


    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CreditsModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

