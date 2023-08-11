use crate::models::{Item, State};
use anyhow::{anyhow, Result};
use web_sys::{console, Storage};

const LOCAL_STORAGE_KEY: &str = "state";

/// Get stored data.
/// Initializes new data if it doesn't exist yet, or if something is wrong with it.
pub fn get() -> State {
    if let Ok(state) = self::get_persisted() {
        state
    } else {
        let items = vec![Item::default()];
        let state = State { items };

        if let Err(error) = set(&state) {
            console::log_2(
                &"Failed to save to local storage".into(),
                &format!("{error:?}").into(),
            );
        }

        state
    }
}

/// Set stored data
pub fn set(items: &State) -> Result<()> {
    let storage = get_local_storage();
    let json = serde_json::to_string(items).unwrap();

    storage
        .set_item(LOCAL_STORAGE_KEY, &json)
        .map_err(|_| anyhow!("Error writing share key to storage"))?;
    Ok(())
}

/// Get the currently stored data.
/// Returns an error result if it doesn't exist, or if something is wrong with it.
fn get_persisted() -> Result<State> {
    let storage = get_local_storage();

    let json = storage
        .get_item(LOCAL_STORAGE_KEY)
        .map_err(|_| anyhow!("Error writing share key to storage"))?
        .ok_or_else(|| anyhow!("No items stores"))?;

    let items: State =
        serde_json::from_str(&json).map_err(|_| anyhow!("Failed to deserialize local storage"))?;
    Ok(items)
}

/// Get an instance to access browser's local storage
fn get_local_storage() -> Storage {
    web_sys::window()
        .expect("no window")
        .local_storage()
        .unwrap()
        .unwrap()
}
