use crate::models::{Item, State, StateAction};
use bounce::use_slice;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
}

#[function_component(ItemInput)]
pub fn item_input(props: &Props) -> Html {
    let input_ref = use_node_ref();
    let state = use_slice::<State>();

    let on_updated = {
        let state = state.clone();

        use_callback(
            move |_, (input_ref, item_id)| {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let value = input.value();
                    state.dispatch(StateAction::Update {
                        id: item_id.to_owned(),
                        value,
                    });
                }
            },
            (input_ref.clone(), props.item.id.clone()),
        )
    };

    html! {
        <input ref={input_ref}
            type="text"
            value={props.item.value.clone()}
            onchange={on_updated}
        />
    }
}
