/* tslint:disable */
/* eslint-disable */
/**
* @returns {number}
*/
export function get_checkerboard_size(): number;
/**
* @returns {number}
*/
export function tick_timeout(): number;
/**
*/
export function start(): void;
/**
* @returns {number}
*/
export function get_output_buffer_pointer(): number;
/**
* @param {number} dark_value_red
* @param {number} dark_value_green
* @param {number} dark_value_blue
* @param {number} light_value_red
* @param {number} light_value_green
* @param {number} light_value_blue
*/
export function generate_checker_board(dark_value_red: number, dark_value_green: number, dark_value_blue: number, light_value_red: number, light_value_green: number, light_value_blue: number): void;
/**
*/
export enum Live {
  Alive,
  Dead,
}
/**
*/
export class Board {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Board}
*/
  static new(width: number, height: number): Board;
/**
* @param {number} w
* @param {number} h
* @param {Cell} cell
*/
  set(w: number, h: number, cell: Cell): void;
/**
* @param {number} w
* @param {number} h
* @returns {boolean}
*/
  is_alive(w: number, h: number): boolean;
/**
* @param {number} w
* @param {number} h
* @returns {number}
*/
  alive_neighbors_count(w: number, h: number): number;
}
/**
*/
export class Cell {
  free(): void;
}
/**
*/
export class Universe {
  free(): void;
/**
* @param {number} width
* @param {number} height
*/
  constructor(width: number, height: number);
/**
*/
  tick(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly start: () => void;
  readonly get_output_buffer_pointer: () => number;
  readonly generate_checker_board: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_cell_free: (a: number) => void;
  readonly __wbg_board_free: (a: number) => void;
  readonly board_new: (a: number, b: number) => number;
  readonly board_set: (a: number, b: number, c: number, d: number) => void;
  readonly board_is_alive: (a: number, b: number, c: number) => number;
  readonly board_alive_neighbors_count: (a: number, b: number, c: number) => number;
  readonly __wbg_universe_free: (a: number) => void;
  readonly universe_new: (a: number, b: number) => number;
  readonly universe_tick: (a: number) => void;
  readonly get_checkerboard_size: () => number;
  readonly tick_timeout: () => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
