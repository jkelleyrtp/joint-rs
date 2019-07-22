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
    crate::elements::canvaselement::CanvasElement,
};




// The state that holds all the user interaction
pub struct Workspace {
    elements: Vec<CanvasElement>,
    user_scrolling: bool,
    shift_down: bool,
}


impl Workspace {
    pub fn new(CANVAS_HEIGHT: f32, CANVAS_WIDTH: f32) -> Workspace {
        // Create the canvas
        Workspace {
            elements: Vec::new(),
            user_scrolling: false,
            shift_down: false,
        }
    }
}


impl Component for Workspace {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Workspace::new(1000.0, 1000.0)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                ConsoleService::new().log("Drag enter");
            },
            Msg::DragEnd => {
                ConsoleService::new().log("Drag ended");
            }
            _ => {
                ConsoleService::new().log("Action not implemented");
            }
        };
        true
    }
}


pub enum Msg {
    Click,
    Scroll,
    MouseWheel,
    AddItem,
    DragEnd
}


impl Renderable<Workspace> for Workspace {
    fn view(&self) -> Html<Self> {
        html! {

            <div class="primaryviewport", 
                onscroll = |_| Msg::AddItem, 
                onmousewheel= |_| Msg::MouseWheel,
                ondragend= |_| Msg::DragEnd,
            >                

                <div class="canvasdroparea",
                    ondragenter=|_| Msg::Click,>

                </div>

            </div>

        }
    }
}



