import {  get_game_frame, main_js, } from "../pkg"
import {GameTicker} from "./GameTicker";
import {Position, Entity} from "./RustTypes"
main_js(); //Setup
let ticker: GameTicker = new GameTicker();
ticker.startTicker();

let gameframe : Entity[] = get_game_frame();
console.log("Printing Game Frame:")
console.dir(gameframe);
console.log("Type:"+gameframe[0].name);