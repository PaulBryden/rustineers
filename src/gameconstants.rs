use wasm_bindgen::prelude::*;
use serde::de::DeserializeOwned;
use serde::ser::Error as SerError;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value, Error, Serializer};
#[derive(Serialize, Deserialize, Clone)]
pub enum Team {
    Team1,
    Team2,
    Team3,
    Team4,
    Neutral,
}
