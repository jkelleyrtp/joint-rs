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




/// Primary element for holding JointElements
pub struct JointCanvas<T: JointElement> {
    elements: Vec<T>,
    user_scrolling: bool,
    shift_down: bool,
}

impl<T: JointElement> JointCanvas<T> {
    pub fn new(CANVAS_HEIGHT: f32, CANVAS_WIDTH: f32) -> JointCanvas<T> {
        // Create the canvas
        JointCanvas {
            elements: Vec::new(),
            user_scrolling: false,
            shift_down: false,
        }
    }
}


impl<T> Component for JointCanvas<T> where T: JointElement+'static {
    type Message = CanvasActions;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        JointCanvas::new(1000.0, 1000.0)
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


pub enum CanvasActions {
    Click,
    Scroll,
    MouseWheel,
    AddItem,
    DragEnd
}


impl<T> Renderable<JointCanvas<T>> for JointCanvas<T> where T: JointElement+'static, Self: Component<Message=CanvasActions>, {
    fn view(&self) -> Html<Self> {
        html! {

            <div class="primaryviewport", 
                onscroll = |_| CanvasActions::AddItem, 
                onmousewheel= |_| CanvasActions::MouseWheel,
                ondragend= |_| CanvasActions::DragEnd,
            >                
                <div class="canvasdroparea",
                    ondragenter=|_| CanvasActions::AddItem,>
                </div>
            </div>

        }
    }
}



