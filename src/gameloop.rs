use wasm_timer::{SystemTime};
use std::thread::sleep;
use std::string;
use wasm_bindgen::prelude::*;
use web_sys::console;
pub use super::gameentity;
pub use super::gameconstants;
pub use super::position;
pub struct Game {
    pub lastTick: f64,
    pub entities: gameentity::EntityContainer<gameentity::EntityID>,
}

impl Game
{   
     pub fn new() -> Game {
    Game { lastTick: 0.0, //milliseconds
        entities: gameentity::EntityContainer::new(),

    }
}

    pub fn reset(& mut self)
    {
        self.lastTick=0.0;
        self.entities.add_entity(gameentity::EntityID::Engineer(gameentity::EngineerEntity::new("Engineer".to_string(), "Test".to_string(), gameconstants::Team::Team1, position::Position::new(0.2,0.3), position::Position::new(0.7,0.3), 37)));

    }
    pub fn GetEntityData(&self) -> Vec<gameentity::Entity>
    {
        self.entities.serialize()
    }

    pub fn tick(&mut self, delta: f64)
    {
        self.lastTick=self.lastTick+delta;
        console::log_1(&JsValue::from_str(self.lastTick.to_string().as_str()));
    }
}