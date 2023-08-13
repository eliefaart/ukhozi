use crate::{components::item_container::ItemContainer, models::State, storage};
use bounce::use_slice;
use yew::prelude::*;

#[function_component(List)]
pub fn list() -> Html {
    let state = use_slice::<State>();

    use_effect_with_deps(
        |state| {
            storage::set(state).expect("Failed to store state in local storage");
            || {}
        },
        (*state).clone(),
    );

    let items = state
        .items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            html! {
                <ItemContainer
                    key={item.id.clone()}
                    item={item.clone()}
                    new_item={idx == state.items.len() - 1}
                />
            }
        })
        .collect::<Html>();

    html! {
        <div class="list">
            {items}
        </div>
    }
}
