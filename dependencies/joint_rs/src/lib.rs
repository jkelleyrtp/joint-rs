
// Elements
// pub mod elements;
use yew :: {
    html, 
    Component, 
    ComponentLink, 
    Html, 
    Renderable, 
    ShouldRender
};
pub mod elements;
pub mod core;


// use core::workspace;

// use elements::canvas;







// // The "source of truth for this app"
// pub struct JointApp {
// }

// pub enum JointStateUpdate {

// }

// impl Component for JointApp {
//     type Message = JointStateUpdate;
//     type Properties = ();

//     fn create(_: Self::Properties,  _: ComponentLink<Self> ) -> Self {
//         JointApp {

//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         true
//     }
    
// }




// impl Renderable<JointApp> for JointApp {
//     fn view(&self) -> Html<Self> {
//         html! {
//             <div name="joint-rs-body", >



//             </div> 
//         }
//     }
// }