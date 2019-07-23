use undo::{Command, Record};
use crate::core::core::JointAppState;
use crate::elements::JointElement;
use std::rc::Rc;



#[derive(Debug)]
pub struct AddElementToGraph<GraphElement: JointElement> {
    element: Rc<GraphElement>,
    key: String
}

impl<GraphElement: JointElement> AddElementToGraph <GraphElement> {
    pub fn new(key: String, value: GraphElement) -> Self {
        Self {
            element: Rc::new(value),
            key: key
        }
    }
}


impl<Element: JointElement> Command<JointAppState<Element>> for AddElementToGraph<Element> {
    fn apply(&mut self, s: &mut JointAppState<Element>) -> undo::Result {
        s.elements.insert(self.key.clone(), self.element.clone());
        Ok(())
    }

    fn undo(&mut self, s: &mut JointAppState<Element>) -> undo::Result {
        s.elements.remove_entry(&self.key);
        // self.things_to_add = s.title.pop().ok_or("`s` is empty")?;
        Ok(())
    }
}












// struct Add()

// impl Command<String> for Add {
//     fn apply(&mut self, s: &mut String) -> undo::Result {
//         s.push(self.0);
//         Ok(())
//     }

//     fn undo(&mut self, s: &mut String) -> undo::Result {
//         self.0 = s.pop().ok_or("`s` is empty")?;
//         Ok(())
//     }
// }


// fn main() -> undo::Result {
//     let mut record = Record::default();
//     record.apply(Add('a'))?;
//     record.apply(Add('b'))?;
//     record.apply(Add('c'))?;
//     assert_eq!(record.as_receiver(), "abc");
//     record.undo().unwrap()?;
//     record.undo().unwrap()?;
//     record.undo().unwrap()?;
//     assert_eq!(record.as_receiver(), "");
//     record.redo().unwrap()?;
//     record.redo().unwrap()?;
//     record.redo().unwrap()?;
//     assert_eq!(record.as_receiver(), "abc");
//     Ok(())
// }