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
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            value: Default::default(),
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
    Remove { id: String },
    Update { id: String, value: String },
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StateAction::Remove { id } => {
                let mut items = self.items.clone();
                items.retain(|i| i.id != id);

                Self { items }.into()
            }
            StateAction::Update { id, value } => {
                let mut items = self.items.clone();

                if let Some(index) = items.iter().position(|i| i.id == id) {
                    items[index].value = value;
                }

                if items.iter().all(|item| !item.value.is_empty()) {
                    items.push(Item::default());
                }

                Self { items }.into()
            }
        }
    }
}
