use wasm_bindgen::prelude::*;
use serde::de::DeserializeOwned;
use serde::ser::Error as SerError;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value, Error, Serializer};

#[derive(Serialize, Deserialize, Clone)]
pub struct Position{
    x: f64,
    y: f64
}
impl Position
{

    pub fn new(x: f64, y: f64) -> Position{
        Position { x:x, y:y}
    }

}