use crate::models::{State, StateAction};
use bounce::use_slice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_id: AttrValue,
}

#[function_component(ItemDelete)]
pub fn item_delete(props: &Props) -> Html {
    let state = use_slice::<State>();

    let on_checked = {
        let state = state.clone();

        use_callback(
            move |_, item_id| {
                state.dispatch(StateAction::Delete {
                    id: item_id.to_string(),
                })
            },
            props.item_id.clone(),
        )
    };

    html! {
        <button onclick={on_checked}
            class="item-delete bg-transparent"
            tabindex="-1"
        >
            <svg viewBox="0 -960 960 960">
                <path d="m249-207-42-42 231-231-231-231 42-42 231 231 231-231 42 42-231 231 231 231-42 42-231-231-231 231Z"/>
            </svg>
        </button>
    }
}
