use std::collections::HashMap;
use std::rc::Rc;
use crate::elements::JointElement;
pub mod commands;
pub mod appstate;
pub mod renderer;

use appstate::JointAppState;
use undo::{Command, Record};





pub struct JointApp<Element: JointElement> where Element: 'static {
    pub command_record: Record<JointAppState<Element>>, 
}


impl<Element: JointElement> JointApp<Element> {
    pub fn new() -> Self {
        let record = Record::<JointAppState<Element>>::default();
        Self {
            command_record: record, 
        }
    }

    pub fn get_elements(&mut self) -> &HashMap<String, Rc<Element>> {
        &self.command_record.as_receiver().elements
    }

    pub fn add_element_to_graph(&mut self, graph_element: Element){
        self.command_record.apply( 
            commands::AddElementToGraph::new( graph_element.get_element_id(), graph_element)
        ).expect("Couldn't add element to graph");
    }

}
