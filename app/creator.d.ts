/* tslint:disable */
/* eslint-disable */
export function init_scene(): boolean;
export function scene_names(): Promise<any>;
export function save_scene(): Promise<any>;
export function load_scene(): Promise<boolean>;
export function delete_scene(): Promise<boolean>;
export function set_scene_name(name: string): boolean;
export function load_first_scene(): Promise<any>;
export function set_material_color(red: string, green: string, blue: string): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_scene: () => [number, number, number];
  readonly scene_names: () => any;
  readonly save_scene: () => any;
  readonly load_scene: () => any;
  readonly delete_scene: () => any;
  readonly set_scene_name: (a: number, b: number) => [number, number, number];
  readonly load_first_scene: () => any;
  readonly set_material_color: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure81_externref_shim_multivalue_shim: (a: number, b: number, c: any) => [number, number];
  readonly closure85_externref_shim: (a: number, b: number, c: any) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6163315e4ab12732: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb953dab231bf761a: (a: number, b: number) => void;
  readonly closure139_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure156_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure180_externref_shim: (a: number, b: number, c: any, d: any) => void;
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
