
use undo::{Record};
use std::collections::HashMap;
use std::rc::Rc;
use crate::elements::JointElement;


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

pub struct JointAppCore<Element: JointElement> {
    pub command_record: Record<JointAppState<Element>>, 
}


impl<Element: JointElement> JointAppCore<Element> {
    pub fn new() -> Self {
        let record = Record::<JointAppState<Element>>::default();
        Self {
            command_record: record, 
        }
    }

    /// Currently not implemented loading yet
    pub fn new_from_saved_state() -> Self {
        let record = Record::<JointAppState<Element>>::default();
        Self {
            command_record: record, 
        }        
    }

    pub fn get_elements(&mut self) -> &HashMap<String, Rc<Element>> {
        &self.command_record.as_receiver().elements
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