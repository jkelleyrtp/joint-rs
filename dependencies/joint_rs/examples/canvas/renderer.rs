#![recursion_limit="256"]

use {
    joint_rs::{
        core::renderer::JointRenderer,
        elements::canvas::JointCanvas,
        elements::menus::JointSidebar,
    },
    yew :: {
        html, 
        Component, 
        ComponentLink, 
        Html, 
        Renderable, 
        ShouldRender
    },
    crate::{
        app::NotionAppInteractions,
        elements::{
            graphelement::NotionElement,
            add_element_button::AddElementButton
        },
        app::NotionApp
    }
};


/// Explicitly draw out how NotionApp will be rendered to the screen
impl Renderable<NotionApp> for NotionApp {
    fn view(&self) -> Html<Self> {
        html! {
        <div class="grid-container", >
            
            <div class="sidebar", >

                // <JointSidebar: />
                // <AddElementButton: />
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 4 } ) }
                { Renderable::view( &Fred { val: 3 } ) }
                { Renderable::view( &Fred { val: 2 } ) }
                { Renderable::view( &Fred { val: 1 } ) }
                { Renderable::view( &Fred { val: 0 } ) }
                { Renderable::view( &Fred { val: 1 } ) }
                { Renderable::view( &Fred { val: 2 } ) }
                { Renderable::view( &Fred { val: 3 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }
                { Renderable::view( &Fred { val: 5 } ) }


            </div>

                < JointCanvas<NotionElement>: />

            <div class="header", > </div>
            
            <div class="footer",> </div>
        </div>            
        }
    }
}

pub struct Fred {
    val: i32
}

impl Renderable<NotionApp> for Fred {
    fn view(&self) -> Html<NotionApp>{
        html! {
            <div> 
                {format!("Hello! My value is: {:?}", self.val) }
            </div>
        }
    }
}