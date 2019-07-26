use {
    euclid,
    yew :: {
        html, 
        Component, 
        ComponentLink, 
        Html, 
        Renderable, 
        ShouldRender,
        services::ConsoleService,
    },
    crate::elements::JointElement,
};




// The state that holds all the user interaction
pub struct Workspace<T: JointElement> {
    elements: Vec<T>,
    user_scrolling: bool,
    shift_down: bool,
}

impl<T: JointElement> Workspace<T> {
    pub fn new(CANVAS_HEIGHT: f32, CANVAS_WIDTH: f32) -> Workspace<T> {
        // Create the canvas
        Workspace {
            elements: Vec::new(),
            user_scrolling: false,
            shift_down: false,
        }
    }
}


impl<T> Component for Workspace<T> where T: JointElement+'static {
    type Message = OtherMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Workspace::new(1000.0, 1000.0)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::Click => {
            //     ConsoleService::new().log("Drag enter");
            // },
            // Msg::DragEnd => {
            //     ConsoleService::new().log("Drag ended");
            // }
            _ => {
                ConsoleService::new().log("Action not implemented");
            }
        };
        true
    }
}


pub enum OtherMsg {
    Click,
    Scroll,
    MouseWheel,
    AddItem,
    DragEnd
}


impl<T> Renderable<Workspace<T>> for Workspace<T> where T: JointElement+'static, Self: Component<Message=OtherMsg>, {

    fn view(&self) -> Html<Self> {
        // let Message = OtherMsg.clone();
        // let additem = OtherMsg::AddItem;
        html! {

            // <div class="primaryviewport", 
            //     onscroll = |_| Message::AddItem, 
            //     onmousewheel= |_| Message::MouseWheel,
            //     ondragend= |_| Message::DragEnd,
            // >                

                <div class="canvasdroparea",
                    ondragenter=|_| OtherMsg::AddItem,>
                </div>

            // </div>

        }
    }
}



