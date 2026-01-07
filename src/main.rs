use relm4::prelude::*;

struct App;

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();
    type Root = ();

    view! {
        gtk::Window {
            set_title: Some("Fractal About Clone"),
            set_default_size: (400, 300),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => Msg::Increment,
                    // ActionablePlus::set_action::<ExampleU8Action>: 1,
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked => Msg::Decrement,
                },

                gtk::Label {
                    set_margin_all: 5,
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                },

                gtk::MenuButton {
                    set_menu_model: Some(&menu_model),
                }
            },
        }
    }
    
    
    fn init_root() -> Self::Root {
        todo!()
    }
    
    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        todo!()
    }
}

fn main() {
    let app = RelmApp::new("org.example.fractal_about_clone");
    app.run::<App>(());
}
