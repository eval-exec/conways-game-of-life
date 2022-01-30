use kiss3d::camera::Camera;
use kiss3d::window::Window;
use rand::random;

#[derive(Clone)]
struct Cell {
    alive: bool,
    birth_time: std::time::SystemTime,

    scene: kiss3d::scene::SceneNode,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            alive: false,
            birth_time: std::time::SystemTime::now(),
            scene: kiss3d::scene::SceneNode::new_empty(),
        }
    }

    fn born(&mut self, wd: &mut kiss3d::window::Window, x: f32, y: f32, z: f32) {
        if !self.alive {
            self.alive = true;
            self.birth_time = std::time::SystemTime::now();
            self.scene = wd.add_cube(0.9, 0.9, 0.9);
            self.scene.set_color(random(), random(), random());
            self.scene
                .set_local_translation(kiss3d::nalgebra::Translation3::new(x, y, z))
        }
    }
    fn kill(&mut self, wd: &mut kiss3d::window::Window, x: f32, y: f32, z: f32) {
        if self.alive {
            self.alive = false;
            wd.remove_node(&mut self.scene);
        }
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

struct Universe {
    len_x: usize,
    len_y: usize,
    len_z: usize,
    universe_idx: usize,
    universe_twin: Vec<Vec<Vec<Vec<Cell>>>>,
}

impl Universe {
    fn new(lenx: usize, leny: usize, lenz: usize) -> Universe {
        let mut u = Universe {
            len_x: lenx,
            len_y: leny,
            len_z: lenz,
            universe_idx: 0,
            universe_twin: vec![],
        };
        u.universe_twin =
            vec![vec![vec![vec![Cell::new(); lenz as usize]; leny as usize]; lenx as usize]; 2];
        u
    }

    fn init(&mut self, wd: &mut kiss3d::window::Window) {
        for ix in 0..self.len_x {
            for iy in 0..self.len_y {
                for iz in 0..self.len_z {
                    if rand::random() {
                        self.universe_twin[0][ix][iy][iz].born(wd, ix as f32, iy as f32, iz as f32);
                    }
                }
            }
        }
    }

    fn tick(&mut self, wd: &mut kiss3d::window::Window) {
        let prev_idx = self.universe_idx;
        let now_idx = (self.universe_idx + 1) % 2;
        for ix in 0..self.len_x {
            for iy in 0..self.len_y {
                for iz in 0..self.len_z {
                    let mut count: u8 = 0;
                    for direction in DIRECTIONS {
                        let x = ix as i32 + direction[0];
                        let y = iy as i32 + direction[1];
                        let z = iz as i32 + direction[2];
                        if x >= 0
                            && x < self.len_x as i32
                            && y >= 0
                            && y < self.len_y as i32
                            && z >= 0
                            && z < self.len_z as i32
                            && self.universe_twin[prev_idx][x as usize][y as usize][z as usize]
                            .alive
                        {
                            count += 1;
                        }
                    }

                    if grim_reaper(self.universe_twin[prev_idx][ix][iy][iz].alive(), count) {
                        self.universe_twin[now_idx][ix][iy][iz]
                            .born(wd, ix as f32, iy as f32, iz as f32)
                    } else {
                        self.universe_twin[now_idx][ix][iy][iz]
                            .kill(wd, ix as f32, iy as f32, iz as f32)
                    }
                }
            }
        }
        self.universe_idx = now_idx;
    }
}

fn grim_reaper(live: bool, count: u8) -> bool {
    match count {
        2 => live,
        3 => true,
        _ => false,
    }
}

const DIRECTIONS: [[i32; 3]; 8 + 9 + 9] = [
    // up
    [0, 0, 1],
    [0, 1, 1],
    [0, -1, 1],
    [1, 0, 1],
    [-1, 0, 1],
    [1, 1, 1],
    [1, -1, 1],
    [-1, -1, 1],
    [-1, 1, 1],
    //middle
    [0, 1, 0],
    [0, -1, 0],
    [1, 0, 0],
    [-1, 0, 0],
    [1, 1, 0],
    [1, -1, 0],
    [-1, 1, 0],
    [-1, -1, 0],
    // down
    [0, 0, -1],
    [0, 1, -1],
    [0, -1, -1],
    [1, 0, -1],
    [-1, 0, -1],
    [1, 1, -1],
    [1, -1, -1],
    [-1, 1, -1],
    [-1, -1, -1],
];

pub fn game_3d() {
    let mut window = kiss3d::window::Window::new("Conway's game of life");
    let lenx = 40;
    let leny = 40;
    let lenz = 1;
    let mut universe = Universe::new(lenx, leny, lenz);
    universe.init(&mut window);

    // let eye = kiss3d::nalgebra::Point3::new(lenx as f32 / 2.0, leny as f32 / 2.0, lenx as f32);
    let eye = kiss3d::nalgebra::Point3::new(lenx as f32 / 2.0, leny as f32 / 2.0, lenx as f32);
    let at = kiss3d::nalgebra::Point3::new(lenx as f32 / 2.0, leny as f32 / 2.0, 0.0);
    let mut first_person = kiss3d::camera::FirstPerson::new(eye, at);
    while window.render_with_camera(&mut first_person) {
        universe.tick(&mut window);
        for mut event in window.events().iter() {
            if let kiss3d::event::WindowEvent::Key(button, kiss3d::event::Action::Press, _) =
            event.value
            {
                if button == kiss3d::event::Key::Q {
                    println!("You pressed the button: {:?}", button);
                    println!("Do not try to press escape: the event is inhibited!");
                    event.inhibited = true; // override the default keyboard handler
                    return;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200))
    }
}
