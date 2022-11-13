use yew::prelude::*;

pub struct Model {
    pub value: i64,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self { value: 0, } }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <body>
                <h2>{ "URCL-rs" }</h2>
                <h3>{ "Another URCL emulator I guess, I don't know, don't ask me" }</h3>
                <p>
                    { "the value of link:" }
                </p>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
