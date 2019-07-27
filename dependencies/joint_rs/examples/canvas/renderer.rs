use {
    joint_rs::{
        render::JointRenderer,
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
        NotionApp,
        app::NotionAppInteractions,
        elements::graphelement::NotionElement
    }
};


impl Component for NotionApp {
    type Message = NotionAppInteractions;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => ()
        };
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

/// Explicitly draw out how NotionApp will be rendered to the screen
impl Renderable<NotionApp> for NotionApp {
    fn view(&self) -> Html<Self> {
        html! {
        <div class="grid-container", >
            
            <div class="sidebar", >

                <JointSidebar: />
                
            </div>

                <JointCanvas<NotionElement>: />

            <div class="header", > </div>
            
            <div class="footer",> </div>
        </div>            
        }
    }
}