use crate::storage;
use bounce::Slice;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub value: String,
    pub checked: bool,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            value: String::new(),
            checked: false,
        }
    }
}

#[derive(Slice, PartialEq, Serialize, Deserialize, Clone)]
pub struct State {
    pub items: Vec<Item>,
}

impl Default for State {
    fn default() -> Self {
        storage::get()
    }
}

pub enum StateAction {
    // Delete an item
    Delete { id: String },
    // Update the value of an item
    SetValue { id: String, value: String },
    // Mark an item as (un)checked
    SetChecked { id: String, checked: bool },
    // Remove non-last empty items from the list
    Clean,
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StateAction::Delete { id } => {
                let mut items = self.items.clone();
                items.retain(|i| i.id != id);

                Self { items }.into()
            }
            StateAction::SetValue { id, value } => {
                let mut items = self.items.clone();

                if let Some(index) = items.iter().position(|i| i.id == id) {
                    items[index].value = value;
                }

                if items.iter().all(|item| !item.value.is_empty()) {
                    items.push(Item::default());
                }

                Self { items }.into()
            }
            StateAction::SetChecked { id, checked } => {
                let mut items = self.items.clone();

                if let Some(index) = items.iter().position(|i| i.id == id) {
                    items[index].checked = checked;
                }

                Self { items }.into()
            }
            StateAction::Clean => {
                let mut items = self.items.clone();
                let item_id_last = items.last().map_or(String::new(), |item| item.id.clone());

                items.retain(|item| !item.value.is_empty() || item.id == item_id_last);

                Self { items }.into()
            }
        }
    }
}
