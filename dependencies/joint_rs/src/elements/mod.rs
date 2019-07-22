#[macro_use]
use yew::prelude::*;
#[macro_use]
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};



pub trait JointCanvas {
}


// Any menu button that you want to employ that interacts with the joint_app
pub trait JointMenuButton {
}


// Any canvas element that you want rendered and placed
// Joint_rs elements can only be yew components
pub trait JointElement: Sized {
    fn get_element_id(&self) -> String; 
 
    // All the element callbacks you can bind to
    fn on_element_remove(&self) {}

    fn on_element_add(&self) {}

    fn on_element_modified(&self) {}
}




// All the default elements
pub mod jointelement;