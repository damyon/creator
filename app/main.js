import init, {
  load_first_scene,
  load_scene,
  delete_scene,
  save_scene,
  init_scene,
  toggle_selection_shape,
  set_material_color,
  scene_names,
  set_scene_name,
  toggle_noise,
  toggle_smooth,
  toggle_solid,
  toggle_fluid,
} from "./creator.js";

const CANVAS_ID = "scene";
const SAVE_ID = "save";
const DELETE_ID = "delete";
const LOAD_ID = "load";
const NAME_ID = "name";
const NOISE_ID = "noise";
const FLAT_ID = "flat";
const SOLID_ID = "solid";
const FLUID_ID = "fluid";
const COLOR_ID = "color";
const ALPHA_ID = "alpha";
const SCENE_LIST_ID = "scene-list";

const SPHERE_ID = "sphere";
const CUBE_ID = "cube";
const SQUARE_XZ_ID = "square-xz";
const SQUARE_XY_ID = "square-xy";
const SQUARE_YZ_ID = "square-yz";
const CIRCLE_XZ_ID = "circle-xz";
const CIRCLE_XY_ID = "circle-xy";
const CIRCLE_YZ_ID = "circle-yz";

var selection_shape = SPHERE_ID;

function get_next_selection_shape() {
  if (selection_shape == SPHERE_ID) return CUBE_ID;
  if (selection_shape == CUBE_ID) return SQUARE_XZ_ID;
  if (selection_shape == SQUARE_XZ_ID) return SQUARE_XY_ID;
  if (selection_shape == SQUARE_XY_ID) return SQUARE_YZ_ID;
  if (selection_shape == SQUARE_YZ_ID) return CIRCLE_XZ_ID;
  if (selection_shape == CIRCLE_XZ_ID) return CIRCLE_XY_ID;
  if (selection_shape == CIRCLE_XY_ID) return CIRCLE_YZ_ID;
  if (selection_shape == CIRCLE_YZ_ID) return SPHERE_ID;
}

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

document.getElementById(FLAT_ID).onclick = function (event) {
  toggle_noise();
  document.getElementById(FLAT_ID).style.display = "none";
  document.getElementById(NOISE_ID).style.display = "block";

  document.getElementById(CANVAS_ID).focus();
};

document.getElementById(NOISE_ID).onclick = function (event) {
  toggle_smooth();
  document.getElementById(NOISE_ID).style.display = "none";
  document.getElementById(FLAT_ID).style.display = "block";

  document.getElementById(CANVAS_ID).focus();
};

document.getElementById(SOLID_ID).onclick = function (event) {
  toggle_solid();
  document.getElementById(SOLID_ID).style.display = "none";
  document.getElementById(FLUID_ID).style.display = "block";

  document.getElementById(CANVAS_ID).focus();
};

document.getElementById(FLUID_ID).onclick = function (event) {
  toggle_fluid();
  document.getElementById(FLUID_ID).style.display = "none";
  document.getElementById(SOLID_ID).style.display = "block";

  document.getElementById(CANVAS_ID).focus();
};

function select_next_shape() {
  toggle_selection_shape();
  selection_shape = get_next_selection_shape();

  document.getElementById(SPHERE_ID).style.display = "none";
  document.getElementById(CUBE_ID).style.display = "none";
  document.getElementById(SQUARE_XZ_ID).style.display = "none";
  document.getElementById(SQUARE_XY_ID).style.display = "none";
  document.getElementById(SQUARE_YZ_ID).style.display = "none";
  document.getElementById(CIRCLE_XZ_ID).style.display = "none";
  document.getElementById(CIRCLE_XY_ID).style.display = "none";
  document.getElementById(CIRCLE_YZ_ID).style.display = "none";

  document.getElementById(selection_shape).style.display = "block";
  document.getElementById(CANVAS_ID).focus();
}

document.getElementById(CANVAS_ID).onkeydown = function (event) {
  // T
  if (event.key == "t") {
    select_next_shape();
    event.preventDefault();
  }
};

document.getElementById(SPHERE_ID).onclick = function (event) {
  select_next_shape();
};
document.getElementById(CUBE_ID).onclick = function (event) {
  select_next_shape();
};
document.getElementById(SQUARE_XZ_ID).onclick = function (event) {
  select_next_shape();
};

document.getElementById(SQUARE_XY_ID).onclick = function (event) {
  select_next_shape();
};

document.getElementById(SQUARE_YZ_ID).onclick = function (event) {
  select_next_shape();
};
document.getElementById(CIRCLE_XZ_ID).onclick = function (event) {
  select_next_shape();
};
document.getElementById(CIRCLE_XY_ID).onclick = function (event) {
  select_next_shape();
};
document.getElementById(CIRCLE_YZ_ID).onclick = function (event) {
  select_next_shape();
};

function updateColour() {
  var hexColor = document.getElementById(COLOR_ID).value;
  var alpha = document.getElementById(ALPHA_ID).value;
  var rgb = hex_to_rgb(hexColor);
  if (rgb) {
    set_material_color(rgb[0] + "", rgb[1] + "", rgb[2] + "", alpha + "");
  }
  document.getElementById(CANVAS_ID).focus();
}
document.getElementById(COLOR_ID).onchange = updateColour;
document.getElementById(ALPHA_ID).onchange = updateColour;

document.getElementById(SCENE_LIST_ID).onchange = function (event) {
  var scene = event.target.value;
  if (scene != "None") {
    var name = document.getElementById(NAME_ID);
    name.value = scene;
    processing = true;
    set_scene_name(scene);
    processing = false;
    setTimeout(load_deferred, 500);
  }
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
    scenes.appendChild(new Option("None"));
    for (const name of values) {
      scenes.appendChild(new Option(name));
    }
  });
}

await load_scene_names();
