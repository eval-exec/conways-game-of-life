use crate::color::color::BLACK;
use kiss3d::scene::SceneNode;

#[derive(PartialEq, Clone, Copy)]
pub enum Live {
    Alive,
    Dead,
}

pub struct Cell {
    live: Live,
}

pub struct Board {
    board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut board: Vec<Vec<Cell>> = Vec::new();
        for _ in 0..height {
            let mut line: Vec<Cell> = Vec::new();
            for w in 0..width {
                line.push(Cell { live: Live::Alive })
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

    fn is_alive(&self, w: usize, h: usize) -> bool {
        self.board[h][w].live == Live::Alive
    }
    fn alive_neighbors_count(&self, w: usize, h: usize) -> u8 {
        let mut count: u8 = 0;
        for dir in DIRECTIONS {
            let mut _w: i32 = w as i32 + dir[0];
            let mut _h = h as i32 + dir[1];
            if _w < 0 {
                _w = self.width as i32 - 1;
            }
            if _w as usize >= self.width {
                _w = 0;
            }
            if _h < 0 {
                _h = self.height as i32 - 1;
            }
            if _h as usize >= self.height {
                _h = 0;
            }
            if self.is_alive(_w as usize, _h as usize) {
                count += 1;
            }
        }
        count
    }

    // calculate board live cells and dead cells count
    fn cell_statics(&mut self) -> (usize, usize) {
        let mut lives: usize = 0;

        for h in 0..self.height {
            for w in 0..self.width {
                if self.board[h][w].live == Live::Alive {
                    lives += 1;
                }
            }
        }
        let deads = self.height * self.width - lives;
        (lives, deads)
    }
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

pub struct Universe {
    twin: Vec<Board>,
    iboard: usize,
    now: u64,
    width: usize,
    height: usize,
}

impl Universe {
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
                let live: Live;
                if fastrand::bool() {
                    live = Live::Alive;
                } else {
                    live = Live::Dead;
                }
                u.twin[0].set(w, h, Cell { live })
            }
        }
        u
    }
    fn get_now_board(&mut self) -> &Board {
        &self.twin[self.iboard]
    }

    fn get_pre_board(&mut self) -> &Board {
        &self.twin[(self.iboard + 1) % 2]
    }
    fn get_board(&mut self, i: usize) -> &Board {
        &self.twin[i]
    }

    pub fn tick(&mut self, mut cf: impl FnMut(usize, usize, Live)) {
        let prev_i = self.iboard;
        let now_i = (prev_i + 1) % 2;
        self.now += 1;
        for h in 0..self.height {
            for w in 0..self.width {
                let mut live: Live;
                match self.get_board(prev_i).alive_neighbors_count(w, h) {
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
                if fastrand::u32(0..100) == 0{
                    live = Live::Alive;
                }
                self.twin[now_i].set(w, h, Cell { live });
                cf(h, w, live)
            }
        }
        self.iboard = now_i;
    }
}
