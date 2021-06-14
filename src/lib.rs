use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
/*#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;*/


// This is like the `main` function, except for JavaScript.

#[wasm_bindgen]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Loaded Rust Module."));

    Ok(())
}

#[wasm_bindgen]
pub fn print_function() {
    
    console::log_1(&JsValue::from_str("Hello from Rust! MODIFIED"));
}

#[wasm_bindgen]
pub struct Person {
    name: String,
}
#[wasm_bindgen]
impl Person {
    pub fn new(val: String) -> Person {
        Person { name: val }
    }

    pub fn get(&self) -> String {
        self.name.clone()
    }

    pub fn set(&mut self, val: String) {
        self.name = val;
    }
}

#[wasm_bindgen]
pub fn new_function(x: Person) {
    console::log_1(&JsValue::from_str("Person's name is:"));
    console::log_1(&JsValue::from_str(&x.name));
}