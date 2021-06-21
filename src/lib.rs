use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::de::DeserializeOwned;
use serde::ser::Error as SerError;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value, Error, Serializer};
use wasm_timer::{SystemTime};
use mut_static::MutStatic;
pub mod gameloop;
pub mod gameentity;
pub mod gameconstants;
pub mod position;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
/*#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;*/


// This is like the `main` function, except for JavaScript.
lazy_static! {
    static ref GAME: MutStatic<gameloop::Game> =  MutStatic::new();
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let sysTime = SystemTime::now();
    GAME.set(gameloop::Game::new()).unwrap();
    GAME.write().unwrap().reset();
    // Your code goes here!
    console::log_1(&JsValue::from_str("Loaded Rust Module."));

    Ok(())
}
#[wasm_bindgen]
pub fn tick(delta: f64) {
        let mut mut_handle = GAME.write().unwrap();
        mut_handle.tick(delta);
        console::log_1(&JsValue::from_str("Ticking."));
}
#[wasm_bindgen]
pub fn print_function() {
    
    console::log_1(&JsValue::from_str("Hello from Rust! MODIFIED"));
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Entity(gameentity::Entity);
#[wasm_bindgen]
pub fn pass_value_to_js() -> Result<JsValue, JsValue> {
	// ...
    let person: Person = Person::new("Test".to_string());
    serde_wasm_bindgen::to_value(&person).map_err(|err| err.into())
}


#[wasm_bindgen]
pub fn serialize_canada_with_serde_json() -> Result<JsValue, JsValue> {
    let mut handle = GAME.read().unwrap();
    let mut entities: Vec<gameentity::Entity> = Vec::new();
    entities = handle.GetEntityData();
	serde_wasm_bindgen::to_value(&entities).map_err(|err| err.into())
}

#[wasm_bindgen]
pub struct Element {
    name: String,
    id: String,
    parent_id: String,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(val: String) -> Person {
        Person { name: val }
    }

    #[wasm_bindgen(getter)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, val: String) {
        self.name = val;
    }
}

#[wasm_bindgen]
pub fn new_function(x: &Person) {
    console::log_1(&JsValue::from_str("Person's name is:"));
    console::log_1(&JsValue::from_str(&x.name));
}