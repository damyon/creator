/* tslint:disable */
/* eslint-disable */
/**
 * Init the scene for the first time.
 */
export function init_scene(): boolean;
/**
 * Get the list of saved scenes.
 */
export function scene_names(): Promise<any>;
/**
 * Save the current scene.
 */
export function save_scene(): Promise<any>;
/**
 * Load the current scene.
 */
export function load_scene(): Promise<boolean>;
/**
 * Switch from fluid to solid material.
 */
export function toggle_fluid(): Promise<boolean>;
/**
 * Switch from solid to fluid material.
 */
export function toggle_solid(): Promise<boolean>;
/**
 * Switch from solid to noise colours.
 */
export function toggle_noise(): Promise<boolean>;
/**
 * Switch from noise to solid colours.
 */
export function toggle_smooth(): Promise<boolean>;
/**
 * Delete the current scene.
 */
export function delete_scene(): Promise<boolean>;
/**
 * Change the name of the scene.
 */
export function set_scene_name(name: string): boolean;
/**
 * Load the default scene when the page loads.
 */
export function load_first_scene(): Promise<any>;
/**
 * Change the selection shape.
 */
export function toggle_selection_shape(): boolean;
/**
 * Change what colour we use.
 */
export function set_material_color(red: string, green: string, blue: string, alpha: string): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_scene: () => [number, number, number];
  readonly scene_names: () => any;
  readonly save_scene: () => any;
  readonly load_scene: () => any;
  readonly toggle_fluid: () => any;
  readonly toggle_solid: () => any;
  readonly toggle_noise: () => any;
  readonly toggle_smooth: () => any;
  readonly delete_scene: () => any;
  readonly set_scene_name: (a: number, b: number) => [number, number, number];
  readonly load_first_scene: () => any;
  readonly toggle_selection_shape: () => [number, number, number];
  readonly set_material_color: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure107_externref_shim_multivalue_shim: (a: number, b: number, c: any) => [number, number];
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h687c20b4a0270c8a: (a: number, b: number) => void;
  readonly closure151_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure156_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure168_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure188_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
