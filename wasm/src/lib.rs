// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.

use getrandom::getrandom;
// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Define the size of our "checkerboard"
pub const CHECKERBOARD_SIZE: usize = 300;
pub const TICKTIMEOUT: usize = 50;

#[wasm_bindgen]
pub fn get_checkerboard_size() -> usize {
    CHECKERBOARD_SIZE
}

#[wasm_bindgen]
pub fn tick_timeout() -> usize {
    TICKTIMEOUT
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(CHECKERBOARD_SIZE as u32);
    canvas.set_height(CHECKERBOARD_SIZE as u32);
}

/*
 * 1. What is going on here?
 * Create a static mutable byte buffer.
 * We will use for putting the output of our graphics,
 * to pass the output to js.
 * NOTE: global `static mut` means we will have "unsafe" code
 * but for passing memory between js and wasm should be fine.
 *
 * 2. Why is the size CHECKERBOARD_SIZE * CHECKERBOARD_SIZE * 4?
 * We want to have 20 pixels by 20 pixels. And 4 colors per pixel (r,g,b,a)
 * Which, the Canvas API Supports.
 */
const OUTPUT_BUFFER_SIZE: usize = CHECKERBOARD_SIZE * CHECKERBOARD_SIZE * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }

    return pointer;
}

// Function to generate our checkerboard, pixel by pixel
#[wasm_bindgen]
pub fn generate_checker_board(
    mut dark_value_red: u8,
    mut dark_value_green: u8,
    mut dark_value_blue: u8,
    mut light_value_red: u8,
    mut light_value_green: u8,
    mut light_value_blue: u8,
) {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf);

    dark_value_red = buf[0];
    dark_value_green = buf[1];
    dark_value_blue = buf[2];
    light_value_red = buf[3];
    light_value_green = buf[4];
    light_value_blue = buf[5];
    // Since Linear memory is a 1 dimensional array, but we want a grid
    // we will be doing 2d to 1d mapping
    // https://softwareengineering.stackexchange.com/questions/212808/treating-a-1d-data-structure-as-2d-grid
    for y in 0..CHECKERBOARD_SIZE {
        for x in 0..CHECKERBOARD_SIZE {
            // Set our default case to be dark squares
            let mut is_dark_square: bool = true;

            // We should change our default case if
            // We are on an odd y
            if y % 2 == 0 {
                is_dark_square = false;
            }

            // Lastly, alternate on our x value
            if x % 2 == 0 {
                is_dark_square = !is_dark_square;
            }

            // Now that we determined if we are dark or light,
            // Let's set our square value
            let mut square_value_red: u8 = dark_value_red;
            let mut square_value_green: u8 = dark_value_green;
            let mut square_value_blue: u8 = dark_value_blue;
            if !is_dark_square {
                square_value_red = light_value_red;
                square_value_green = light_value_green;
                square_value_blue = light_value_blue;
            }

            // Let's calculate our index, using our 2d -> 1d mapping.
            // And then multiple by 4, for each pixel property (r,g,b,a).
            let square_number: usize = y * CHECKERBOARD_SIZE + x;
            let square_rgba_index: usize = square_number * 4;

            // Finally store the values.
            unsafe {
                OUTPUT_BUFFER[square_rgba_index + 0] = square_value_red; // Red
                OUTPUT_BUFFER[square_rgba_index + 1] = square_value_green; // Green
                OUTPUT_BUFFER[square_rgba_index + 2] = square_value_blue; // Blue
                OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
            }
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Live {
    Alive,
    Dead,
}

#[wasm_bindgen]
pub struct Cell {
    live: Live,
    birth_day: u64,
}

#[wasm_bindgen]
pub struct Board {
    board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

pub const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[wasm_bindgen]
impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut board: Vec<Vec<Cell>> = Vec::new();
        for _ in 0..height {
            let mut line: Vec<Cell> = Vec::new();
            for w in 0..width {
                line.push(Cell {
                    live: Live::Alive,
                    birth_day: 0,
                })
            }
            board.push(line);
        }
        Board {
            board,
            width,
            height,
        }
    }
    pub fn set(&mut self, w: usize, h: usize, cell: Cell) {
        self.board[h][w] = cell
    }

    pub fn is_alive(&self, w: usize, h: usize) -> bool {
        self.board[h][w].live == Live::Alive
    }
    pub fn alive_neighbors_count(&self, w: usize, h: usize) -> u8 {
        let mut count: u8 = 0;
        for dir in DIRECTIONS {
            let _w: i32 = w as i32 + dir[0];
            let _h = h as i32 + dir[1];
            if _w < 0 || _w as usize >= self.width || _h < 0 || _h as usize >= self.height {
                continue;
            }
            if self.is_alive(_w as usize, _h as usize) {
                count += 1;
            }
        }
        count
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub struct Universe {
    twin: Vec<Board>,
    iboard: usize,
    now: u64,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Universe {
        let mut u: Universe = Universe {
            iboard: 0,
            twin: vec![],
            now: 0,
            width: width,
            height: height,
        };
        u.twin.push(Board::new(width, height));
        u.twin.push(Board::new(width, height));

        for h in 0..height {
            for w in 0..width {
                let mut buf = [0u8; 1];
                getrandom::getrandom(&mut buf);

                let mut live: Live = Live::Alive;
                if buf[0] % 2 == 1 {
                    live = Live::Dead;
                }
                u.twin[0].set(w, h, Cell { live, birth_day: 0 })
            }
        }
        u
    }
    fn get_board(&self, i: usize) -> &Board {
        &self.twin[i]
    }

    pub fn tick(&mut self) {
        let prev_i = self.iboard;
        let now_i = (prev_i + 1) % 2;
        self.now += 1;

        let mut buf = [0u8; 10];
        getrandom::getrandom(&mut buf).unwrap();
        let timenow = chrono::Utc::now().timestamp_millis() as u64;

        for h in 0..self.height {
            for w in 0..self.width {
                let mut live: Live;
                let neighbors = self.get_board(prev_i).alive_neighbors_count(w, h);
                match neighbors {
                    0..=1 => {
                        live = Live::Dead;
                    }
                    3 => {
                        live = Live::Alive;
                    }
                    4..=u8::MAX => {
                        live = Live::Dead;
                    }
                    _ => {
                        if self.get_board(prev_i).is_alive(w, h) {
                            live = Live::Alive;
                        } else {
                            live = Live::Dead;
                        }
                    }
                }
                //
                if live == Live::Dead
                    && timenow
                        % (CHECKERBOARD_SIZE + (w + 1) * (h + 1) + 9873 + neighbors as usize) as u64
                        == 0
                {
                    live = Live::Alive;
                }
                self.twin[now_i].set(
                    w,
                    h,
                    Cell {
                        live,
                        birth_day: self.now,
                    },
                )
            }
        }
        self.iboard = now_i;

        unsafe {
            for h in 0..self.height {
                for w in 0..self.width {
                    let square_number: usize = h * CHECKERBOARD_SIZE + w;
                    let square_rgba_index: usize = square_number * 4;
                    let n = (w + h) % 3;

                    if self.get_board(now_i).is_alive(w, h) {
                        if self.get_board(prev_i).is_alive(w, h) {
                        } else {
                            OUTPUT_BUFFER[square_rgba_index] = buf[n]; // Red
                            OUTPUT_BUFFER[square_rgba_index + 1] = buf[n + 1]; // Green
                            OUTPUT_BUFFER[square_rgba_index + 2] = buf[n + 2]; // Blue
                            OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
                        }
                    } else {
                        OUTPUT_BUFFER[square_rgba_index] = 0; // Red
                        OUTPUT_BUFFER[square_rgba_index + 1] = 0; // Green
                        OUTPUT_BUFFER[square_rgba_index + 2] = 0; // Blue
                        OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
                    }
                }
            }
        }
    }
}
