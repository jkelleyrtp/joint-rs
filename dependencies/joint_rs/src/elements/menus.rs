use yew::{html, Callback, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;


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
        <li class="card_items" ,ondragstart=|_| Msg::Startdrag,ondragend=|_| Msg::Enddrag, >
            <div class="view",  > 
                <p >
                    {"Nothing for you !"}
                </p>
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





