import init, { draw_grid } from "./creator.js";

const CANVAS_ID = "triangle";
var other = 10;

async function run() {
  await init();
  var count_str = (other++) + "";
  
  draw_grid(CANVAS_ID, count_str);

  setTimeout(run, 60);
}

run();
