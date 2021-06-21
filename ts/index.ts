import { Person, new_function, pass_value_to_js, tick, main_js, serialize_canada_with_serde_json} from "../pkg"
import {GameTicker} from "./GameTicker";
let person: Person = new Person("Test 2");
new_function(person);
person.name = "Blah";
new_function(person);
main_js(); //Setup
let ticker: GameTicker = new GameTicker();
ticker.startTicker();

var test = serialize_canada_with_serde_json();
console.dir(test);