use crate::{
    components::{item_check::ItemCheck, item_delete::ItemDelete, ItemInput},
    models::Item,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
    #[prop_or(false)]
    pub draft: bool,
}

#[function_component(ItemContainer)]
pub fn item_container(props: &Props) -> Html {
    let class = classes!("item", if props.item.checked { "checked" } else { "" });

    html! {
        <div class={class}>
            if !props.draft {
                <ItemCheck item={props.item.clone()}/>
            }
            else {
                <span class="filler"></span>
            }

            <ItemInput item={props.item.clone()}/>

            if !props.draft {
                <ItemDelete item_id={props.item.id.clone()}/>
            }
            else {
                <span class="filler"></span>
            }
        </div>
    }
}
