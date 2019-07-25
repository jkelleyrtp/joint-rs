use {
    joint_rs::render::JointRenderer,
    yew :: {
        html, 
        Component, 
        ComponentLink, 
        Html, 
        Renderable, 
        ShouldRender
    },
    crate::NotionApp,
    crate::app::NotionAppInteractions
};


impl Component for NotionApp {
    type Message = NotionAppInteractions;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => ()
        };
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

