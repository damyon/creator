import init, {
  load_first_scene,
  load_scene,
  delete_scene,
  save_scene,
  init_scene,
  set_material_color,
  scene_names,
  set_scene_name,
  toggle_noise,
  toggle_smooth,
} from "./creator.js";

const CANVAS_ID = "scene";
const SAVE_ID = "save";
const DELETE_ID = "delete";
const LOAD_ID = "load";
const NAME_ID = "name";
const NOISE_ID = "noise";
const SOLID_ID = "solid";
const COLOUR_ID = "colour";
const SCENE_LIST_ID = "scene-list";

var processing = true;

document.getElementById(SAVE_ID).onclick = function () {
  processing = true;
  save_scene();
  load_scene_names();
  processing = false;
};

document.getElementById(DELETE_ID).onclick = function () {
  processing = true;
  delete_scene();
  load_scene_names();
  processing = false;
};

function load_deferred() {
  load_scene();
  processing = false;
}

document.getElementById(LOAD_ID).onclick = function () {
  processing = true;
  setTimeout(load_deferred, 500);
};

document.getElementById(NAME_ID).onchange = function (e) {
  processing = true;
  set_scene_name(e.target.value);
  processing = false;
};

function hex_to_rgb(hex) {
  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result ? [parseInt(result[1], 16), parseInt(result[2], 16), parseInt(result[3], 16)] : null;
}

document.getElementById(NOISE_ID).onclick = function (event) {
  toggle_noise();
  document.getElementById(NOISE_ID).style.display = "none";
  document.getElementById(SOLID_ID).style.display = "block";

  document.getElementById(CANVAS_ID).focus();
};

document.getElementById(SOLID_ID).onclick = function (event) {
  toggle_smooth();
  document.getElementById(SOLID_ID).style.display = "none";
  document.getElementById(NOISE_ID).style.display = "block";

  document.getElementById(CANVAS_ID).focus();
};

document.getElementById(COLOUR_ID).onchange = function (event) {
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
    let scenes = document.getElementById(SCENE_LIST_ID);
    scenes.innerHTML = "";
    for (const name of values) {
      scenes.appendChild(new Option(name));
    }
  });
}

await load_scene_names();
