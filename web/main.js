import init, { draw_scene, init_scene, set_material_color } from "./creator.js";

const CANVAS_ID = "scene";
var other = 10;

async function run() {
  
  draw_scene(CANVAS_ID);

  setTimeout(run, 100);
}

function hex_to_rgb(hex) {
  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result ? [
    parseInt(result[1], 16),
    parseInt(result[2], 16),
    parseInt(result[3], 16)
   ] : null;
}


document.getElementById("color").onchange = function (event) {
  var hexColor = event.target.value;
  var rgb = hex_to_rgb(hexColor);
  if (rgb) {
    set_material_color(rgb[0] + "", rgb[1] + "", rgb[2] + "");
  }
  document.getElementById(CANVAS_ID).focus();
};


await init();

await init_scene(CANVAS_ID);

run();
