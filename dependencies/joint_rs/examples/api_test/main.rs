use joint_rs::core::workspace::JointApp;
use joint_rs::core::workspace::JointElement;


struct MyJointElement {
    title: String,
}

impl MyJointElement {
    fn new (name: String) -> Self{
        Self {
            title: name,
        }
    }
}

impl JointElement for MyJointElement {
    fn print_name(&self) {
        println!("My name is {}", self.title);
    }
}


fn main() {
    let mut jointapp = JointApp::<MyJointElement>::new();

    // let names = &["Bill", "Joe", "Fred", "Phil"];
    
    // for name in names.iter(){
    //    jointapp.add_element(MyJointElement::new(name.to_string())) 
    // }

    jointapp.print_elements();

}