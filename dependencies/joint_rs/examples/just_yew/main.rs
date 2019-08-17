#![recursion_limit="512"]
use {
    yew::{ Renderable, Html, html, App, run_loop, Component, ComponentLink, ShouldRender},
    yew::services::ConsoleService,
};


fn main() {

    let my_app = App::<NotionApp>::new();
    my_app.mount_to_body();
    run_loop();

}









pub struct NotionElement{

}




pub struct NotionApp {
    // link: ComponentLink<>
    elements: Vec<NotionElement>,
    user_scrolling: bool,
    shift_down: bool,
}


pub enum NotionAppInteractions {
    DragElement,
    OpenInNotion,
    Click,
    Scroll,
    MouseWheel,
    AddItem,
    DragEnd    
}

impl Component for NotionApp {
    type Message = NotionAppInteractions;
    type Properties = ();

    /// Default back to the standard new function
    fn create(_: Self::Properties, parent: ComponentLink<Self>) -> Self { 
        Self {
            elements: Vec::new(),
            user_scrolling: false,
            shift_down: false,
            // joint_core: JointApp::<NotionElement>::new(),
            // link: parent
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





impl Renderable<NotionApp> for NotionApp {
    fn view(&self) -> Html<Self> {
        html! {
        <div class="grid-container", >
            
            <div class="sidebar", >
                

            </div>

                <div class="primaryviewport", 
                    onscroll = |_| NotionAppInteractions::AddItem, 
                    onmousewheel= |_| NotionAppInteractions::MouseWheel,
                    ondragend= |_| NotionAppInteractions::DragEnd,
                >                
                    <div class="canvasdroparea",
                        ondragenter=|_| NotionAppInteractions::AddItem,>
                    </div>
                </div>

            <div class="header", > </div>
            
            <div class="footer",> </div>
        </div>            
        }
    }
}