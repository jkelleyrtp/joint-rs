use yew::{
    Html,
    Renderable,
    Component
};

use std::collections::HashMap;
use crate::elements::JointElement;



pub struct JointApp<Element: JointElement> {
    elements: HashMap<String, Element>
}

impl<Element: JointElement> JointApp<Element> {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new()
        }
    }

    pub fn add_element( &mut self, element: Element ) {
        let key = element.get_element_id();
        self.elements.insert(key, element);
    }

    pub fn remove_element(&mut self, key: &String) -> Option<Element> {
        self.elements.remove(key)
    } 
}



