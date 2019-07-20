// #[macro_use]
// extern crate yew;
// use yew::prelude::*;
// use yew::{ 
//     html, 
//     Component, 
//     ComponentLink, 
//     Html, 
//     Renderable, 
//     ShouldRender
// };
use {
    euclid,
    yew :: {
        html, 
        Component, 
        ComponentLink, 
        Html, 
        Renderable, 
        ShouldRender
    },

};
use euclid;
// use yew::prelude::*;
use yew::services::ConsoleService;

// ================================== //
// Supprting equipment for the canvas //
// ================================== //


// Type of canvas movement implemented
pub enum CanvasType {
    // Allows infinite movement in both x and y directions
    // Make sure you actually need infinite 
    // TODO: Implement this
    InfiniteXY,

    // Allows infinite movement in just x
    // Make sure you actually need infinite 
    // TODO: Implement this
    InfiniteX,

    // Allows infinite movement in just y
    // Make sure you actually need infinite 
    // TODO: Implement this
    InfiniteY,

    // Canvas is of a fixed size 
    Fixed
}

// The coodinate system in the DOM canvas uses euclid internally
// Give us units to work with later on
pub struct ScreenSpace;
pub type ScreenPoint = euclid::TypedPoint2D<f32, ScreenSpace>;
pub type ScreenSize = euclid::TypedSize2D<f32, ScreenSpace>;



// A canvas contains all the elements 

pub struct Canvas {
    canvastype: CanvasType,
    elements: Vec<CanvasElement>,
    viewport: euclid::TypedBox2D<f32, ScreenSpace>,
    canvasbox: euclid::TypedBox2D<f32, ScreenSpace>,
    value: i64,
}


impl Canvas {
    pub fn new( CANVAS_HEIGHT: f32, CANVAS_WIDTH: f32 ) -> Canvas {

        // Define the canvas 
        let canvasbox = euclid::TypedBox2D::from_points(
            &[ScreenPoint::new(0.0, 0.0), ScreenPoint::new(CANVAS_WIDTH, CANVAS_HEIGHT)]
        );

        // Build a defualt viewport for the canvas
        let viewport = euclid::TypedBox2D::from_points(
            &[ScreenPoint::new(-800.0, -800.0), ScreenPoint::new(800.0, 800.0)]
        );

        // Create the canvas
        Canvas { 
            canvastype: CanvasType::Fixed,
            elements: Vec::new(),
            viewport: viewport,
            canvasbox: canvasbox,
            value: 0,
        }
    }


    pub fn set_canvas_size(&self, width: f32, height: f32) {
        // &self.canvasbox.inflate(width, height);

    }

    pub fn set_viewport_size(&self, width: f32, height: f32) {
        // &self.viewport.inflate(width, height);
    }



}



pub enum Msg {
    Click,
}



impl Component for Canvas {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Canvas::new(1000.0, 1000.0)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                ConsoleService::new().log("Drag enter");
                self.
            }
        }
        true
    }
}


impl Renderable<Canvas> for Canvas {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="canvasdroparea", ondragenter=|_| Msg::Click, >


            </div>
        }
    }
}



pub struct CanvasElement {
    name: String,
    hashid: String,
    data: String,
}


#[test]
fn canvas_test() {
    // let workspace_canvas = Canvas::new();

}

#[test]
fn yew_test() {
    // yew::start_app::<Canvas>();
}




