/// Import our functionality from the supporting files
mod elements;
mod renderer;
mod app;
mod context;

use {
    joint_rs,
    elements :: {
        graphelement::NotionElement
    },
    renderer :: {
    },
    app:: {
        NotionApp
    },
    yew::{
        Renderable,
        Html,
        html
    }
};

/// Explicitly draw out how NotionApp will be rendered to the screen
impl Renderable<NotionApp> for NotionApp {
    fn view(&self) -> Html<Self> {
        html! {
        <div class="grid-container", >
            
            <div class="sidebar", >

                // <SidebarModel: />
                
            </div>

            // <JointrsWorkspace: />

            <div class="header", > </div>
            
            <div class="footer",> </div>
        </div>            
        }
    }
}

fn main() {
    // Our NotionApp is using the NotionElement
    let notion_app = NotionApp::new();

    // let notion_app = DefaultApp::<MyElement>::new();

    // yew::start_app::<JointrsDemo>()
}

