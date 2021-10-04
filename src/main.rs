use std::borrow::Borrow;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use rand;
use rand::Rng;


#[derive(PartialEq)]
enum Live {
    Alive,
    Dead,
}

struct Cell {
    live: Live,
    birth_day: u64,
}

struct Board {
    board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}


const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut board: Vec<Vec<Cell>> = Vec::new();
        for i in 0..height {
            let mut line: Vec<Cell> = Vec::new();
            for w in 0..width {
                line.push(Cell { live: Live::Alive, birth_day: 0 })
            }
            board.push(line);
        }
        Board {
            board,
            width,
            height,
        }
    }
    fn set(&mut self, w: usize, h: usize, cell: Cell) {
        self.board[h][w] = cell
    }

    fn is_alive(&self, w: usize, h: usize) -> bool {
        self.board[h][w].live == Live::Alive
    }
    fn alive_neighbors_count(&self, w: usize, h: usize) -> u8 {
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


struct Universe {
    twin: Vec<Board>,
    iboard: usize,
    now: u64,
    width: usize,
    height: usize,

}

impl Universe {
    fn new(width: usize, height: usize) -> Universe {
        let mut u: Universe = Universe { iboard: 0, twin: vec![], now: 0, width: width, height: height };
        u.twin.push(Board::new(width, height));
        u.twin.push(Board::new(width, height));

        for h in 0..height {
            for w in 0..width {
                let count = u.getBoard(0).alive_neighbors_count(w, h);
                let mut live: Live;
                if rand::thread_rng().gen_bool(1.0 / 2.0) {
                    live = Live::Alive;
                } else {
                    live = Live::Dead;
                }
                u.twin[0].set(w, h, Cell { live, birth_day: 0 })
            }
        }
        u
    }
    fn getNowBoard(&mut self) -> &Board {
        &self.twin[self.iboard]
    }
    fn getBoard(&mut self, i: usize) -> &Board {
        &self.twin[i]
    }
    fn tick(&mut self) {
        let prev_i = self.iboard;
        let now_i = (prev_i + 1) % 2;
        self.now += 1;
        for h in 0..self.height {
            for w in 0..self.width {
                let mut live: Live;
                match self.getBoard(prev_i).alive_neighbors_count(w, h) {
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
                        if self.getBoard(prev_i).is_alive(w, h) {
                            live = Live::Alive;
                        } else {
                            live = Live::Dead;
                        }
                    }
                }
                self.twin[now_i].set(w, h, Cell { live, birth_day: self.now })
            }
        }
        self.iboard = now_i;
    }
}


fn main() {
    let mut width: usize = 0;
    let mut height: usize = 0;

    match termion::terminal_size() {
        Ok(size) => {
            width = size.0 as usize;
            height = size.1 as usize;
        }
        Err(err) => {
            panic!(err)
        }
    }
    let mut universe: Universe = Universe::new(width, height);


    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(stdout,
             "{}{}{}Use the up/down arrow keys to change the blue in the rainbow.",
             termion::clear::All,
             termion::cursor::Goto(1, 1),
             termion::cursor::Hide)
        .unwrap();

    writeln!(stdout, "{}", termion::clear::All).unwrap();
    loop {
        universe.tick();

        let board = universe.getNowBoard();
        for h in 0..board.height {
            writeln!(stdout, "{}", termion::cursor::Goto(1, h as u16 + 1)).unwrap();
            for w in 0..board.width {
                if board.is_alive(w, h) {
                    write!(stdout, "◼").unwrap();
                } else {
                    write!(stdout, " ").unwrap();
                }
            }
        }
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
