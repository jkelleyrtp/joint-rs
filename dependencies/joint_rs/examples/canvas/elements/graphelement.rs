use joint_rs::elements::JointElement;

use yew :: {
    html, 
    Component, 
    ComponentLink, 
    Html, 
    Renderable, 
    ShouldRender
};

use crate :: {
    app::NotionAppInteractions as Updates
};

pub struct NotionElement {
    element_id: String
}

impl NotionElement {
    fn new(id: String) -> Self {
        Self {
            element_id: id
        }
    }    
}

impl JointElement for NotionElement {
    fn get_element_id(&self) -> String {
        self.element_id.clone()
    }
}



impl Component for NotionElement {
    type Message = Updates;
    type Properties = ();

    fn create(_: Self::Properties,  _: ComponentLink<Self> ) -> Self {
        Self {
            element_id: "asdasd".to_string()
        }        
    }

    fn update(&mut self, update_id: Updates) -> ShouldRender {
        true
    }
}

// pub enum Updates {
//     Remove,
//     EditTitle,
//     ChangeColor,
//     OpenInNotion
// }    


impl Renderable<NotionElement> for NotionElement {
    fn view(&self) -> Html<Self> {
        html! {
            <div name="notion_element", style="background: red", >
                <button onclick=|_| Updates::OpenInNotion, > { "Open in Notion" } </button>
                {"This is a notion element1 "}
                {"This is a notion element2 "}
                {"This is a notion element3 "}
            </div> 
        }
    }
}


// impl JointElement for NotionElement {
//     fn get_element_id(&self) -> String {
//         "wasd".to_string()
//     }

// }