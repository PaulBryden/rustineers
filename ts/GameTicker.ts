import { Person, new_function, pass_value_to_js, tick, main_js } from "../pkg"

export class GameTicker
{

    lastTime: number;
    run: boolean;

    constructor() {
        this.lastTime = performance.now();
        this.run=false;
      }

      startTicker()
      {
        this.run=true;
        this.lastTime=performance.now();
        this.tickGame(this.lastTime);
      }

      stopTicker()
      {
        this.run=false;
      }

    tickGame(currentTime: number)
    {
        if(this.run)
        {
            let delta: number = currentTime-this.lastTime; this.lastTime = currentTime;  tick(delta);  
            requestAnimationFrame((tick)=>{this.tickGame(tick)});
        }
    };
};