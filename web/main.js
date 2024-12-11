import init, { draw_grid } from "./creator.js";

const CANVAS_ID = "triangle";
var other = 10;

async function run() {
  await init();
  
  draw_grid(CANVAS_ID);

  setTimeout(run, 60);
}

run();
