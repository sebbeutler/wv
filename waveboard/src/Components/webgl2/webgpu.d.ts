/* tslint:disable */
/* eslint-disable */
export function surfboard(canvas_id: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly surfboard: (a: number, b: number) => void;
  readonly main: (a: number, b: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_1: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly closure109_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure111_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly closure107_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure317_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure319_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure315_externref_shim: (a: number, b: number, c: any) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h353d168720c16ab2: (a: number, b: number) => void;
  readonly closure311_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure313_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure639_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure648_externref_shim: (a: number, b: number, c: any) => void;
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
