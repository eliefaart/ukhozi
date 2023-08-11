use bounce::BounceRoot;
use components::*;
use yew::prelude::*;

mod components;
mod models;
mod storage;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BounceRoot>
            <List/>
        </BounceRoot>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
