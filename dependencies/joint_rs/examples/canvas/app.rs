use {
    joint_rs::{
        elements::canvas::JointCanvas,
        elements::menus::JointSidebar,
        core::JointApp
    },
    yew :: { html, Component, ComponentLink, Html, Renderable, ShouldRender,
            services::ConsoleService},
    yew::worker::*,
    serde_derive::*,
    crate::{
        elements::graphelement::NotionElement
    }
};

pub struct NotionApp {
    joint_core: JointApp<NotionElement>,
    link: ComponentLink<NotionApp>
}


pub enum NotionAppInteractions {
    DragElement,
    OpenInNotion
}

impl Component for NotionApp {
    type Message = NotionAppInteractions;
    type Properties = ();

    /// Default back to the standard new function
    fn create(_: Self::Properties, parent: ComponentLink<Self>) -> Self { 
        Self {
            joint_core: JointApp::<NotionElement>::new(),
            link: parent
        }
    }

    // Start wiring our app together
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NotionAppInteractions::OpenInNotion => {
                ConsoleService::new().log("Button pressed!");
            },
            _ => ()
        };
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
}












// #[derive(Serialize, Deserialize, Debug)]
// pub enum Request {
//     Question(String),
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Response {
//     Answer(String),
// }

// impl Agent for NotionApp {
//     // Available:
//     // - `Job` (one per bridge)
//     // - `Context` (shared in the same thread)
//     // - `Public` (separate thread).
//     type Reach = Context; // Spawn only one instance per thread (all components could reach this)
//     type Message = NotionAppInteractions;
//     type Input = Request;
//     type Output = Response;

//     // Create an instance with a link to agent's environment.
//     fn create(link: AgentLink<Self>) -> Self {
//         NotionApp::new()
//     }

//     // Handle inner messages (of services of `send_back` callbacks)
//     fn update(&mut self, msg: Self::Message) { /* ... */ }

//     // Handle incoming messages from components of other agents.
//     fn handle(&mut self, msg: Self::Input, who: HandlerId) {
//         match msg {
//             Request::Question(_) => {
//                 self.link.response(who, Response::Answer("That's cool!".into()));
//             },
//         }
//     }
// }