import init, {
  load_first_scene,
  load_scene,
  simplify_scene,
  delete_scene,
  save_scene,
  init_scene,
  set_material_color,
  scene_names,
  set_scene_name,
} from "./creator.js";

const CANVAS_ID = "scene";
var processing = true;

document.getElementById("save").onclick = function () {
  processing = true;
  save_scene();
  load_scene_names();
  document.getElementById(CANVAS_ID).focus();
  processing = false;
};

document.getElementById("delete").onclick = function () {
  processing = true;
  delete_scene();
  load_scene_names();
  document.getElementById(CANVAS_ID).focus();
  processing = false;
};

document.getElementById("simplify").onclick = function () {
  processing = true;
  simplify_scene();
  document.getElementById(CANVAS_ID).focus();
  processing = false;
};

function load_deferred() {
  load_scene();
  processing = false;
}

document.getElementById("load").onclick = function () {
  processing = true;
  setTimeout(load_deferred, 500);
  document.getElementById(CANVAS_ID).focus();
};

document.getElementById("name").onchange = function (e) {
  processing = true;
  set_scene_name(e.target.value);
  processing = false;
};

function hex_to_rgb(hex) {
  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result ? [parseInt(result[1], 16), parseInt(result[2], 16), parseInt(result[3], 16)] : null;
}

document.getElementById("color").onchange = function (event) {
  var hexColor = event.target.value;
  var rgb = hex_to_rgb(hexColor);
  if (rgb) {
    set_material_color(rgb[0] + "", rgb[1] + "", rgb[2] + "");
  }
  document.getElementById(CANVAS_ID).focus();
};

document.getElementById(CANVAS_ID).width = window.innerWidth;
document.getElementById(CANVAS_ID).height = window.innerHeight;

await init();

await init_scene(CANVAS_ID);

await load_first_scene();

async function load_scene_names() {
  let process = scene_names();

  process.then((values) => {
    let scenes = document.getElementById("scene-list");
    scenes.innerHTML = "";
    for (const name of values) {
      scenes.appendChild(new Option(name));
    }
  });
}

await load_scene_names();
