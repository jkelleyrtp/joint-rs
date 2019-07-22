// Default implementation of a JointElement
use super::JointElement;

use yew :: {
    html, 
    Component, 
    ComponentLink, 
    Html, 
    Renderable, 
    ShouldRender
};


pub struct DefaultJointElement {

}

impl JointElement for DefaultJointElement {
    fn get_element_id(&self) -> String {
        "asdad".to_string()
    }
}