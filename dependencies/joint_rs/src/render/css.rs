// Manages how styling for each component can be accessed through external css








struct CSS {
    id: &str,
    class: &str,   
}


impl CSS {
    fn new() -> CSS {

    }

    fn set_id(self, id: &str) -> CSS {
        self.id = id;
        self
    }

    fn set_class(self, id: &str) -> CSS {
        self.id = id;
        self
    }

    // Generates a single page html style guide for all the components in 
    fn style_guide () {



    }

}

