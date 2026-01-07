use relm4::prelude::*;

struct App;

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Fractal About Clone"),
            set_default_size: (400, 300),
        }
    }
    
    type Root;
    
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
