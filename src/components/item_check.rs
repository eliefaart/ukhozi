use crate::models::{Item, State, StateAction};
use bounce::use_slice;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
}

#[function_component(ItemCheck)]
pub fn item_check(props: &Props) -> Html {
    let state = use_slice::<State>();

    let on_checked = {
        let state = state.clone();

        use_callback(
            move |_, item| {
                state.dispatch(StateAction::SetChecked {
                    id: item.id.to_string(),
                    checked: !item.checked,
                })
            },
            props.item.clone(),
        )
    };

    let mut class = classes!("item-check");
    if props.item.checked {
        class.push("checked");
    }

    html! {
        <button onclick={on_checked}
            class="item-check"
            tabindex="-1"
        >
            if props.item.checked {
                <svg viewBox="0 -960 960 960">
                    <path d="M378-246 154-470l43-43 181 181 384-384 43 43-427 427Z"/>
                </svg>
            }
        </button>
    }
}
