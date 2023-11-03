/* tslint:disable */
/* eslint-disable */
/**
* @param {string} password
* @returns {any}
*/
export function generate_y(password: string): any;
/**
* @param {string} k
* @returns {any}
*/
export function generate_r(k: string): any;
/**
* @param {string} k
* @param {string} c
* @param {string} password
* @returns {any}
*/
export function find_solve(k: string, c: string, password: string): any;
/**
* @returns {any}
*/
export function gen_random_below(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate_y: (a: number, b: number) => number;
  readonly generate_r: (a: number, b: number) => number;
  readonly find_solve: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly gen_random_below: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
