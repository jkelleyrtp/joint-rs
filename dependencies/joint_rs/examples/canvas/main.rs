/// Import our functionality from the supporting files
mod elements;
mod renderer;
mod app;
mod context;

use {
    joint_rs :: {
        elements::canvas::JointCanvas,
        elements::menus::JointSidebar,
        core::JointApp,
    },
    elements :: {
        graphelement::NotionElement,
    },
    renderer :: {
    },
    yew::{
        Renderable,
        Html,
        html,
        App,
        run_loop,
    },
    crate::{
        app::NotionApp
    },
};


fn main() {
    // let renderer = NotionRenderer::new();

    // let notion_app = JointApp::<NotionElement, NotionRenderer>::new(renderer);
    // yew::start_app::<NotionRenderer>();

    let my_app = App::<NotionApp>::new();
    my_app.mount_to_body();
    run_loop();
    
}



