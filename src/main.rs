use yew::prelude::*;
mod bindings;



pub struct Model {
    pub value: i64,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { bindings::log("Created"); Self { value: 0, } }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <body>
                <nav>
                    <button id="red">{ "ERRORS" }</button>
                    <button id="green">{ "COMPILE" }</button>
                    <button id="yellow">{ "EXAMPLES" }</button>
                    <button id="blue">{ "DOCUMENTATION" }</button>
                    <button id="magenta">{ "SETTINGS" }</button>
                </nav>
                <main>
                    <textarea></textarea>
                </main>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
