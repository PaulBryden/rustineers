use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::FileReader;
use serde::de::DeserializeOwned;
use serde::ser::Error as SerError;
use serde::{Deserialize, Serialize};
use serde_json::{ Value};

use serde_wasm_bindgen::{from_value, to_value, Error, Serializer};
use wasm_timer::{SystemTime};
use mut_static::MutStatic;
use include_dir::Dir;
use include_dir::include_dir;
use std::path::Path;
use tiledmap::TiledMap;
pub mod gameloop;
pub mod gameentity;
pub mod gameconstants;
pub mod position;
pub mod tiledmap;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        static PROJECT_DIR: Dir = include_dir!("map");

        // of course, you can retrieve a file by its full path
        let lib_rs = PROJECT_DIR.get_file("tiledmap3.json").unwrap();
        
        // you can also inspect the file's contents
        let body = lib_rs.contents_utf8().unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map: serde_json::Value = serde_json::from_str(&body).unwrap();
        let mapCast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let test = mapCast.tilesets[0].tiles.as_ref().unwrap()[0].id;
        println!("{}",test);
    }
}

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
    GAME.set(gameloop::Game::new()).unwrap();
    GAME.write().unwrap().reset();  
    static PROJECT_DIR: Dir = include_dir!("map");

    // of course, you can retrieve a file by its full path
    let lib_rs = PROJECT_DIR.get_file("tiledmap3.json").unwrap();
    
    // you can also inspect the file's contents
    let body = lib_rs.contents_utf8().unwrap();
    console::log_1(&JsValue::from_str(body));
    let body = lib_rs.contents_utf8().unwrap();
    console::log_1(&JsValue::from_str(body));
    let map: serde_json::Value = serde_json::from_str(&body).unwrap();
    console::log_1(&JsValue::from_str("About to cast to JSON"));
    let mapCast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
    // Your code goes here!
    Ok(())
}

#[wasm_bindgen]
pub fn tick(delta: f64) {
        let mut mut_handle = GAME.write().unwrap();
        mut_handle.tick(delta);
        console::log_1(&JsValue::from_str("Ticking."));
}

#[wasm_bindgen]
pub fn get_game_frame() -> Result<JsValue, JsValue> {
    let handle = GAME.read().unwrap();
	serde_wasm_bindgen::to_value(&handle.GetEntityData()).map_err(|err| err.into())
}