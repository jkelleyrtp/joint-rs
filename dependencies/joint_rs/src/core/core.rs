// use yew::{
//     Html,
//     Renderable,
//     Component
// };

use undo::{Record};

use std::collections::HashMap;
use std::rc::Rc;
use crate::elements::JointElement;
// use crate::elements;
// use crate::core::commands;


/// The application state that gets managed by the command queue system
pub struct JointAppState<Element: JointElement> {
    pub title: String,
    pub elements: HashMap<String, Rc<Element>>
}

impl<Element: JointElement> JointAppState<Element> {
    fn new() -> Self {
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
    fn new_from_saved_state() -> Self {
        let record = Record::<JointAppState<Element>>::default();
        Self {
            command_record: record, 
        }        
    }

    pub fn get_elements(&mut self) -> &HashMap<String, Rc<Element>> {
        &self.command_record.as_receiver().elements
    }
}
