/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function start(): void;
export function get_output_buffer_pointer(): number;
export function generate_checker_board(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function __wbg_cell_free(a: number): void;
export function __wbg_board_free(a: number): void;
export function board_new(a: number, b: number): number;
export function board_set(a: number, b: number, c: number, d: number): void;
export function board_is_alive(a: number, b: number, c: number): number;
export function board_alive_neighbors_count(a: number, b: number, c: number): number;
export function __wbg_universe_free(a: number): void;
export function universe_new(a: number, b: number): number;
export function universe_tick(a: number): void;
export function get_checkerboard_size(): number;
export function tick_timeout(): number;
export function __wbindgen_exn_store(a: number): void;
export function __wbindgen_start(): void;
