// What about the jointrs workspace is exposed to the outside world





pub struct JointApp<T: JointElement> {
    elements: Vec<T>,
}

impl<T: JointElement> JointApp<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element( &mut self, elemnt: T ) {
        self.elements.push(elemnt);
    }

    pub fn print_elements(&self) {
        for element in self.elements.iter(){
            element.print_name();
        }
    }


}




pub trait JointElement: Sized {
    fn print_name(&self); 
}



