use joint_rs :: {
    core::JointApp,
    elements::jointelement::DefaultJointElement,
    elements,
    core
};





// fn main() {

//     // let mut jointapp = core::DefaultJointApp::new();

//     // let sidebar = elements::

//     // let mut renderer = 

//     // let mut jointapp = JointApp::<MyJointElement>::new();

// }

use undo::{Command, Record};

struct Add(char);

impl Command<String> for Add {
    fn apply(&mut self, s: &mut String) -> undo::Result {
        s.push(self.0);
        Ok(())
    }

    fn undo(&mut self, s: &mut String) -> undo::Result {
        self.0 = s.pop().ok_or("`s` is empty")?;
        Ok(())
    }
}


fn main() -> undo::Result {
    let mut record = Record::default();
    record.apply(Add('a'))?;
    record.apply(Add('b'))?;
    record.apply(Add('c'))?;
    assert_eq!(record.as_receiver(), "abc");
    record.undo().unwrap()?;
    record.undo().unwrap()?;
    record.undo().unwrap()?;
    assert_eq!(record.as_receiver(), "");
    record.redo().unwrap()?;
    record.redo().unwrap()?;
    record.redo().unwrap()?;
    assert_eq!(record.as_receiver(), "abc");
    Ok(())
}