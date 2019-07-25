use joint_rs;

use joint_rs::core::commands;
use joint_rs::core::core;
use joint_rs::elements::JointElement;
use joint_rs::core::JointApp;

use std::collections::HashMap;
use std::rc::Rc;

pub struct my_joint_element {
    element_id: String
}

impl my_joint_element {
    fn new(id: String) -> Self {
        Self {
            element_id: id
        }
    }
}


impl JointElement for my_joint_element {
    fn get_element_id(&self) -> String {
        self.element_id.clone()
    }
}

pub struct my_joint_app{
    joint_core: core::JointAppCore<my_joint_element>,
}

impl JointApp for my_joint_app {
    type element = my_joint_element;
    type interactions = interactions;

    fn get_mut_element_list(&mut self) -> &HashMap<String, Rc<Self::element>> {
        self.joint_core.get_elements()
    }
}

impl my_joint_app {
    pub fn new() -> Self {
        Self {
            joint_core: core::JointAppCore::<my_joint_element>::new(),
        }
    }

    pub fn handle_interaction(&mut self, interaction: interactions) {
        match interaction {
            interactions::AddElement(element) => {
                self.joint_core.command_record.apply(
                    commands::AddElementToGraph::new(
                        element.get_element_id(),
                        element
                    )
                ).expect("Couldn't add element to graph");
            }
        }
    }
}


pub enum interactions {
    AddElement(my_joint_element)
}






fn main()  {

    // let mut appcore = core::JointAppCore::<my_joint_element>::new();

    let mut this_joint_app = my_joint_app::new();
    this_joint_app.handle_interaction(
        interactions::AddElement(
            my_joint_element::new("Element1".into())
        )
    );





    // println!("\nElements on canvas: ");
    // for (key, value) in appcore.get_elements().iter() {
    //     print!("Key: {:?} , ", key.clone(),);
    //     print!("Value: {:?} |  ", value.get_element_id(),);
    // }



    // let mut record = Record::default();
    // record.apply(Add::new('a'))?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // record.apply(Add::new('b'))?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // record.apply(Add::new('c'))?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // // assert_eq!(record.as_receiver(), "abc");
    // record.undo().unwrap()?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // record.undo().unwrap()?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // record.undo().unwrap()?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // // assert_eq!(record.as_receiver(), "");
    // record.redo().unwrap()?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // record.redo().unwrap()?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // record.redo().unwrap()?;
    // println!("Current state of the record: {:?} ", record.as_receiver()) ;
    // // assert_eq!(record.as_receiver(), "abc");
    // println!("All good");
    
    
    // Ok(())
}