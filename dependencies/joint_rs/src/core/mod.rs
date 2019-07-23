use yew::{
    Html,
    Renderable,
    Component
};

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


    // /// Handle anything with the spceific focus on rolling back commands
    // fn handle_interactions(&mut self, interaction: Self::interactions);

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


/// A default JointApp that should be good for the majority of use cases.
pub struct DefaultJointApp {
    elements: HashMap<String, DefaultJointElement>,

}


impl DefaultJointApp {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new()
        }
    }



    /// Pass in json and 
    fn load_elements_from_json(&self) {

    }


}

pub enum Interactions {
    AddElement

}


// impl JointApp for DefaultJointApp {
//     type element = DefaultJointElement;
//     type interactions = Interactions;

//     /// Handle interacitons
//     // fn handle_interactions(&mut self, interaction: Self::interactions) {
//     //     match interaction {
//     //         Interactions::AddElement => {

//     //         },
//     //     };

//     // }


//     /// Need to provide the trait some way to access the hashamp
//     fn get_mut_element_list(&mut self) -> &HashMap<String, Rc<Self::element>> {
//         &mut self.elements
//     }

// }
