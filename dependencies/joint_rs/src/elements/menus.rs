#![recursion_limit="512"]

// use serde_derive::{Deserialize, Serialize};
// use strum::IntoEnumIteratr;
// use strum_macros::{EnumIter, ToString};
use yew::{html, Callback, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
// use yew::events::IKeyboardEvent;
// use yew::format::Json;
// use yew::services::storage::{StorageService, Area};

// struct Menu {

 
// }



// struct MenuItem {
//     css_class: &str,
//     css_id: &str,
    
// }





// ====================== //
// The scrollable sidebar //
// ====================== //
pub struct SidebarState {
    entries: Vec<SidebarListItem>,
    console: ConsoleService,
}

pub struct SidebarModel {
    state: SidebarState
}

pub enum Msg {
    AddItem,
    Startdrag,
    Enddrag,
    // ItemDragging(usize)
    ItemDragging
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Send Signal".into(),
            onsignal: None,
        }
    }
}

impl Component for SidebarModel {
    type Message = Msg;
    type Properties = Props;
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = SidebarState {
            entries: Vec::new(),
            console: ConsoleService::new(),
        };



        Self {
            state
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem => {
                let entry = SidebarListItem {
                    title: "nothing".to_string()
                };
                self.state.entries.push(entry);
                self.state.console.log("Added item");

            },

            Msg::ItemDragging => {
                self.state.console.log("Drag started!");

            },

            Msg::Startdrag => {
                self.state.console.log("Drag started!");
            },

            Msg::Enddrag => {
                self.state.console.log("Drag stopped");


            }

        }
        true
    }
}

impl Renderable<SidebarModel> for SidebarModel {
    fn view(&self) -> Html<Self> {






        html! {



            <>
                <div class="panel-header", >
                    <button onclick=|_| Msg::AddItem, > { "add item to list" } </button>
                    <p class="sidebartitle",> { "Hello World" } </p>
                </div>

                <div class="panel-body",>
                    <div class="list-group",>
                        <ul class="todo-list",>
                            { for self.state.entries.iter().enumerate().map(view_entry) }
                        </ul>
                    </div>
                </div>
            </>
        }
    }
}



pub struct SidebarListItem {
    title: String
}



pub fn view_entry((idx, entry): (usize, &SidebarListItem)) -> Html<SidebarModel> {

    html! {
        <li class="card_items" , >
            <div class="view",> 
                <p ondragstart=|_| Msg::Startdrag, >
                    {"Nothing for you !"}
                </p>
                // <input class="toggle" type="checkbox" checked=entry.completed onclick=|_| Msg::Toggle(idx) />
                // <label ondoubleclick=|_| Msg::ToggleEdit(idx)>{ &entry.description }</label>
                // <button class="destroy" onclick=|_| Msg::Remove(idx) />
            </div>
        </li>
    }
}




// impl Component for SidebarListItem {
//     type Message =  Msg;
//     type Properties = ();
//     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//         Self {
//             title: "hlelr;lej".to_string(),
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::DoIt => {

//             }
//         }
//         true
//     }
// }





