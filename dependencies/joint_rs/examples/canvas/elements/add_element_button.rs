use yew :: { html, Component, ComponentLink, Html, Renderable, ShouldRender };
use crate::app::NotionAppInteractions as NotionUpdates;


/// Adds elements to the canvas
pub struct AddElementButton {
    parent: ComponentLink<Self>
}


impl Component for AddElementButton {
    type Message = Updates;
    type Properties = ();

    fn create(_: Self::Properties,  parent: ComponentLink<Self> ) -> Self {
        // let callback = parent.send_back(|_| Updates::EditTitle);
        
        Self {
            parent: parent
        }        
    }

    fn update(&mut self, update_id: Updates) -> ShouldRender {
        match update_id {
            Updates::Remove => {
                self.parent.send_back( Updates::EditTitle );

                // self.parent.send_self(Updates::EditTitle);
                // self.parent.send_back(function: F)
                // self.parent.scope.send_message(NotionUpdates::OpenInNotion);
            },
            _ => ()
        }
        true
    }
}

pub enum Updates {
    Remove,
    EditTitle(NotionUpdates),
    ChangeColor,
}    


impl Renderable<AddElementButton> for AddElementButton {
    fn view(&self) -> Html<Self> {
        html! {
            <div name="notion_element", style="background: red", >
                <button onclick=|_| Updates::Remove, > { "Open in Notion" } </button>
                {"This is a notion element1 "}
                {"This is a notion element2 "}
                {"This is a notion element3 "}
            </div> 
        }
    }
}