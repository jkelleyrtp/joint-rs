
// use {
//     yew:: {
//         worker::AgentLink,
//         worker::Agent,
//         worker::*
//     },
//     crate::{
//         NotionApp
//     }
// };


// struct Worker {
//     link: AgentLink<Worker>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Request {
//     Question(String),
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Response {
//     Answer(String),
// }

// impl Agent for Worker {
//     // Available:
//     // - `Job` (one per bridge)
//     // - `Context` (shared in the same thread)
//     // - `Public` (separate thread).
//     type Reach = Context; // Spawn only one instance per thread (all components could reach this)
//     type Message = Msg;
//     type Input = Request;
//     type Output = Response;

//     // Create an instance with a link to agent's environment.
//     fn create(link: AgentLink<Self>) -> Self {
//         Worker { link }
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



// struct Model {
//     context: Box<Bridge<context::Worker>>,
// }

// enum Msg {
//     ContextMsg(context::Response),
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let callback = link.send_back(|_| Msg::ContextMsg);
//         // `Worker::bridge` spawns an instance if no one is available
//         let context = context::Worker::bridge(callback); // Connected! :tada:
//         Model { context }
//     }
// }
