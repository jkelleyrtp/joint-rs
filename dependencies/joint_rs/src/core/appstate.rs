
use undo::{Record};
use std::collections::HashMap;
use std::rc::Rc;
use crate::elements::JointElement;
use super::commands;


/// The application state that gets managed by the command queue system
pub struct JointAppState<Element: JointElement> {
    pub title: String,
    pub elements: HashMap<String, Rc<Element>>
}

impl<Element: JointElement> JointAppState<Element> {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            elements: HashMap::new()
        }
    }
}

/// Default app state for a new blank state.
impl <Element: JointElement> Default for JointAppState<Element> {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            elements: HashMap::new()
        }
    }
}




    // pub fn add_element<T: JointElement> (&mut self, element: T) {
    //     self.command_record.apply(
    //         commands::AddElementToGraph::new(
    //             element.get_element_id(),
    //             element
    //         )
    //     ).expect("Couldn't add element to graph"); // need to handle this better
    // }