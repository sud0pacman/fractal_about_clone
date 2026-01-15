use relm4::{adw, gtk::prelude::*, prelude::*};

mod dialog;
use dialog::{AwesomeModel, DialogMsg};

#[derive(Debug)]
enum AppMsg {
    AwesomeModel,
    Close,
}

struct App {
    dialog: Controller<AwesomeModel>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        adw::ApplicationWindow
        {
            set_default_size: (500, 250),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                // adw::HeaderBar {
                //     // set_titlebar: Some(model.header.widget()),

                // },
                adw::ToolbarView {
                    set_top_bar_style: adw::ToolbarStyle::Flat,

                    add_top_bar = &adw::HeaderBar {
                        // #[wrap(Some)]
                        // set_title_widget = &adw::WindowTitle {
                        //     set_title: "Wifi",
                        // }
                    },
                    gtk::LinkButton {
                        set_label: "hi",
                        connect_clicked => AppMsg::AwesomeModel,
                    },
                },

            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dialog = AwesomeModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| AppMsg::Close);

        let model = App { dialog };

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

