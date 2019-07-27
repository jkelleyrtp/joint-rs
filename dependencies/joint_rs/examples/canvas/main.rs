/// Import our functionality from the supporting files
mod elements;
mod renderer;
mod app;
mod context;

use {
    joint_rs :: {
        elements::canvas::JointCanvas,
        elements::menus::JointSidebar,
    },
    elements :: {
        graphelement::NotionElement,
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

fn main() {
    let notion_app = yew::start_app::<NotionApp>();
}

