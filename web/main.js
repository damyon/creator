import init, { draw_scene, init_logger } from "./creator.js";

const CANVAS_ID = "scene";
var other = 10;

async function run() {
  
  draw_scene(CANVAS_ID);

  setTimeout(run, 60);
}
await init();

await init_logger();

run();
