use crate::models::{State, StateAction};
use bounce::use_slice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_id: AttrValue,
}

#[function_component(ItemCheck)]
pub fn item_check(props: &Props) -> Html {
    let state = use_slice::<State>();

    let on_checked = {
        let state = state.clone();

        use_callback(
            move |_, item_id| {
                state.dispatch(StateAction::Remove {
                    id: item_id.to_string(),
                })
            },
            props.item_id.clone(),
        )
    };

    html! {
        <button onclick={on_checked}
            tabindex="-1"
        >
            <svg viewBox="0 0 24 24"
                fill="none"
            >
                <path d="M4 12.6111L8.92308 17.5L20 6.5"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        </button>
    }
}
