use crate::{
    components::{item_check::ItemCheck, ItemInput},
    models::Item,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
    #[prop_or(true)]
    pub deletable: bool,
}

#[function_component(ItemContainer)]
pub fn item_container(props: &Props) -> Html {
    html! {
        <div class="item">
            <ItemInput item={props.item.clone()}/>

            if props.deletable {
                <ItemCheck item_id={props.item.id.clone()}/>
            }
            else {
                <span></span>
            }
        </div>
    }
}
