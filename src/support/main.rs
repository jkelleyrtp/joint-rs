use yew::{
    html, 
    Component, 
    ComponentLink, 
    Html, 
    Renderable, 
    ShouldRender,
    services::ConsoleService
};

struct Model {
    console: ConsoleService
 }

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { 
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                self.console.log("Hello world");
                true
                
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            // <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
            // <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>


        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
