// Some beautiful 2018 rust imports here
use {
    std :: {
        collections::HashMap,
        rc::Rc
    },
    joint_rs :: {
        core::JointApp,
        elements::JointElement,
        core::commands,
        core::core::JointAppCore,
    },
    crate :: {
        elements::graphelement::NotionElement,
    }
};

pub struct NotionApp {
    joint_core: JointAppCore<NotionElement>,
}

impl NotionApp {
    pub fn new() -> Self {
        Self {
            joint_core: JointAppCore::<NotionElement>::new(),
        }
    }

    pub fn handle_interaction(&mut self, interaction: NotionAppInteractions) {
        match interaction {
            NotionAppInteractions::AddElement(element) => {
                self.joint_core.command_record.apply(
                    commands::AddElementToGraph::new(
                        element.get_element_id(),
                        element
                    )
                ).expect("Couldn't add element to graph"); // need to handle this better
            },
            _ => ()
        }
    }
}

impl JointApp for NotionApp {
    type element = NotionElement;
    type interactions = NotionAppInteractions;

    fn get_mut_element_list(&mut self) -> &HashMap<String, Rc<Self::element>> {
        self.joint_core.get_elements()
    }
}

type element_id = String;

pub enum NotionAppInteractions {
    AddElement(NotionElement),
    SidebarDragging(element_id),
    SidebarElementReleased(element_id),

}

