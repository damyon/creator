import init, { draw_grid } from "./creator.js";

const CANVAS_ID = "triangle";

async function run() {
  await init();
  
  draw_grid(CANVAS_ID);
}

run();
