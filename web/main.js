import init, { draw_scene, init_scene } from "./creator.js";

const CANVAS_ID = "scene";
var other = 10;

async function run() {
  
  draw_scene(CANVAS_ID);

  setTimeout(run, 100);
}
await init();

await init_scene(CANVAS_ID);

run();
