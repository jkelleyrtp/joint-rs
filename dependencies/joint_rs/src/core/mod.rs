use std::collections::HashMap;
use std::rc::Rc;
use crate::elements::JointElement;
use crate::elements::jointelement::DefaultJointElement;

pub mod commands;
pub mod core;



/// Required functionality to override the default JointApp implementation
///
/// Useful when you want to start overriding the default logic and element system
pub trait JointApp {
    type element: JointElement;
    type interactions: 'static;

    /// Get the list of elemenents
    fn get_mut_element_list(&mut self) -> &HashMap<String, Rc<Self::element>>;

    // /// Add an element 
    // fn add_element(&mut self, element: Self::element) {
    //     let key = element.get_element_id(); 
    //     self.get_mut_element_list().insert(key, element);
    // }

    // /// Remove elements from the entire scope 
    // fn remove_element(&mut self, key: &String) -> Option<Self::element> {
    //     self.get_mut_element_list().remove(key)
    // }


}

