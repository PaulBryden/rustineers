export class Position{
    x: number;
    y: number;
}
export class Entity {
     name: String;
    id: String;
    team: number;
    position: Position;
    velocity: Position;
    health: number;
}