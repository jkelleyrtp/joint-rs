// use yew::prelude::*;
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
extern crate joint_rs;
use joint_rs::elements::menus::SidebarModel;
use joint_rs::elements::canvas::Canvas;

pub struct ModelState {
}

pub struct Model {
    state: ModelState
}



pub enum Msg {
    AddItem,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = ModelState {
            // entries: Vec::new(),
        };

        Self {
            state
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem => {

            },

        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // <div class="custom-components-example", >
            //     <TESTELEMENT: title="Middle" ,/>
            // </div>        // <SidebarModel />

        <div class="grid-container", >
            
            <div class="sidebar", >

                <SidebarModel: />
                
            </div>

            <div class="primaryviewport", >
                <Canvas: />
            </div>

            <div class="header", >
            
            </div>

              <div class="footer",></div>

        </div>            
        }
    }
}





fn main() {
    // yew::initialize();
    // yew::App::new();



    // App::<joint_rs::elements::canvas::Canvas>::new().mount_to_body();
    // yew::run_loop();


    // yew::start_app::<joint_rs::elements::menus::SidebarModel>()
    yew::start_app::<Model>()
}

