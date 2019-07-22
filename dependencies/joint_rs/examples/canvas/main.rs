// use yew::prelude::*;
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender, services::ConsoleService,};
extern crate joint_rs;
use joint_rs::elements::menus::SidebarModel;
use joint_rs::elements::canvas::Workspace as JointrsWorkspace;

mod notionelement;
use notionelement::NotionElement;


pub struct JointrsDemo {
}

pub enum Msg {

}

impl Component for JointrsDemo {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {};
        false
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<JointrsDemo> for JointrsDemo {
    fn view(&self) -> Html<Self> {
        html! {
            // <div class="custom-components-example", >
            //     <TESTELEMENT: title="Middle" ,/>
            // </div>        // <SidebarModel />

        <div class="grid-container", >
            
            <div class="sidebar", >

                <SidebarModel: />
                
            </div>

            <JointrsWorkspace: />

            <div class="header", > </div>
            
            <div class="footer",> </div>
        </div>            
        }
    }
}







fn main() {
    yew::start_app::<JointrsDemo>()
}

