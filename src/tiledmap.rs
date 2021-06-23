use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize, Debug)]
pub struct TiledMap
{
    compressionlevel: i64,
    editorsettings: EditorSettings,
    height: i64,
    infinite: bool,
    layers: Vec<LayerData>,
    nextlayerid: i64,
    nextobjectid: i64,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i64,
    pub tilesets: Vec<TileSet>,
    tilewidth: i64,
    #[serde(rename="type")]
    tiled_type: String,
    version: f64,
    width: i64

}
#[derive(Serialize,Deserialize, Debug)]
struct EditorSettings
{
    export: Export
}
#[derive(Serialize,Deserialize, Debug)]
struct Export
{
    format: String,
    target: String
}
#[derive(Serialize,Deserialize, Debug)]
struct LayerData
{
    data: Vec<i64>,
    height: i64,
    pub id: i64,
    name: String,
    opacity: i64,
    #[serde(rename="type")]
    tiled_type: String,
    visible: bool,
    width: i64,
    x: i64,
    y: i64
}
#[derive(Serialize,Deserialize,  Debug)]
pub struct TileSet
{
    columns: i64,
    firstgid: i64,
    image: String,
    imageheight: i64,
    imagewidth: i64,
    margin: i64,
    name: String,
    spacing: i64,
    tilecount: i64,
    tileheight: i64,
    #[serde(default)]
    pub tiles: Option<Vec<Tile>>,
    tilewidth: i64,
}
#[derive(Serialize,Deserialize, Debug)]
pub struct Tile
{
    pub id: i64,
    properties: Vec<Property>
}
#[derive(Serialize,Deserialize, Debug)]
struct Property
{
    name: String,
    #[serde(rename="type")]
    tiled_type: String,
    value: bool
}