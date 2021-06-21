use wasm_bindgen::prelude::*;
use serde::de::DeserializeOwned;
use serde::ser::Error as SerError;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value, Error, Serializer};
pub use super::gameconstants;
pub use super::position;

pub trait EntityType : Sync + Send {
    fn tick(&self);
    fn serialize(&self) -> Entity;
}

pub struct EngineerEntity {
    pub data: Entity,
}

impl EntityType for EngineerEntity
{
    fn tick(&self)
    {
        //Update State Machine
    }
    fn serialize(&self) -> Entity
    {
        self.data.clone()
    }
}

impl EngineerEntity
{
    
    pub fn new(name: String, id: String, team: gameconstants::Team, position: position::Position, velocity: position::Position, health: u32) -> EngineerEntity
    {
        EngineerEntity { data: Entity { name: name, id: id, team: team, position: position, velocity: velocity, health: health }}
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub name: String,
    pub id: String,
    pub team: gameconstants::Team,
    pub position: position::Position,
    pub velocity: position::Position,
    pub health: u32,

}

impl Entity
{

    pub fn new(name: String, id: String, team: gameconstants::Team, position: position::Position, velocity: position::Position, health: u32) -> Entity{
        Entity { name: name, id: id, team: team, position: position, velocity: velocity, health: health }
    }

}