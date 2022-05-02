use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::resource::{Effect, Material, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::scene::{ObjectData, SceneNode};
use kiss3d::window::Window;
use nalgebra::{Isometry3, Matrix3, Matrix4, Point3, Translation3, UnitQuaternion, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
enum Live {
    Alive,
    Dead,
}

trait Node {
    fn live(&self) -> bool;
    fn birth_day(&self) -> u64;
    fn toggle(&mut self);
}

#[derive(Clone)]
struct Cell {
    live: Live,
    birth_day: u64,
    sceneNode: SceneNode,
}

struct Universe {
    length: usize,
    cells: Vec<Vec<Vec<Cell>>>,
}

impl Cell {
    fn new() -> Self {
        Cell {
            live: Live::Alive,
            birth_day: 0,
            sceneNode: SceneNode::new_empty(),
        }
    }
}

impl Node for Cell {
    fn live(&self) -> bool {
        self.live == Live::Alive
    }

    fn birth_day(&self) -> u64 {
        self.birth_day
    }

    fn toggle(&mut self) {
        match self.live {
            Live::Alive => self.live = Live::Dead,
            Live::Dead => self.live = Live::Alive,
        }
    }
}

impl Universe {
    fn new(length: usize) -> Universe {
        let cells = vec![vec![vec![Cell::new(); length]; length]; length];
        Universe { length, cells }
    }

    fn set_cell(&mut self, x: usize, y: usize, z: usize, cell: Cell) {
        self.cells[x][y][z] = cell;
    }

    fn tick(&mut self) {
        for x in 0..self.length {
            for y in 0..self.length {
                self.cells[x][y][0].toggle();
                self.cells[x][y][self.length - 1].toggle();
            }
        }
    }
}

pub fn cube() {
    let mut window = Window::new_with_size("Kiss3d: cube", 1920, 1080);
    window.set_light(Light::StickToCamera);
    let mut group = window.add_group();

    let mut c = group.add_cube(1.0, 1.0, 1.0);
    c.set_color(1.0, 0.0, 0.0);

    // let mut q = group.add_cube(100.0, 100.0, 0.0);
    // q.set_local_translation(nalgebra::Translation3::new(0.0, 0.0, 50.0));

    let material = Rc::new(RefCell::new(
        Box::new(NormalMaterial::new()) as Box<dyn Material + 'static>
    ));
    c.set_material(material);

    let length: f32 = 125.0;

    let eye = kiss3d::nalgebra::Point3::new(length, length, length);
    let at = kiss3d::nalgebra::Point3::new(0.0, 0.0, 0.0);
    let mut first_person = kiss3d::camera::FirstPerson::new(eye, at);

    let z = nalgebra::Point3::new(0.0, 0.0, 0.0);
    let a = nalgebra::Point3::new(1000.0, 0.0, 0.0);
    let b = nalgebra::Point3::new(0.0, 1000.0, 0.0);
    let c = nalgebra::Point3::new(0.0, 0.0, 1000.0);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    while window.render_with_camera(&mut first_person) {
        group.prepend_to_local_rotation(&rot);

        window.draw_line(&z, &a, &nalgebra::Point3::new(1.0, 0.0, 0.0));
        window.draw_line(&z, &b, &nalgebra::Point3::new(0.0, 1.0, 0.0));
        window.draw_line(&z, &c, &nalgebra::Point3::new(0.0, 0.0, 1.0));
    }
}

// A material that draws normals
pub struct NormalMaterial {
    shader: Effect,
    position: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
}

impl NormalMaterial {
    pub fn new() -> NormalMaterial {
        let mut shader = Effect::new_from_str(NORMAL_VERTEX_SRC, NORMAL_FRAGMENT_SRC);

        shader.use_program();

        NormalMaterial {
            position: shader.get_attrib("position").unwrap(),
            normal: shader.get_attrib("normal").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            scale: shader.get_uniform("scale").unwrap(),
            view: shader.get_uniform("view").unwrap(),
            proj: shader.get_uniform("proj").unwrap(),
            shader,
        }
    }
}

impl Material for NormalMaterial {
    fn render(
        &mut self,
        pass: usize,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        camera: &mut dyn Camera,
        _: &Light,
        _: &ObjectData,
        mesh: &mut Mesh,
    ) {
        self.shader.use_program();
        self.position.enable();
        self.normal.enable();

        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        self.transform.upload(&formated_transform);
        self.scale.upload(&formated_scale);

        mesh.bind_coords(&mut self.position);
        mesh.bind_normals(&mut self.normal);
        mesh.bind_faces();

        Context::get().draw_elements(
            Context::TRIANGLES,
            mesh.num_pts() as i32,
            Context::UNSIGNED_SHORT,
            0,
        );

        mesh.unbind();

        self.position.disable();
        self.normal.disable();
    }
}

static NORMAL_VERTEX_SRC: &str = "#version 100
attribute vec3 position;
attribute vec3 normal;
uniform mat4 view;
uniform mat4 proj;
uniform mat4 transform;
uniform mat3 scale;
varying vec3 ls_normal;

void main() {
    ls_normal   = normal;
    gl_Position = proj * view * transform * mat4(scale) * vec4(position, 1.0);
}
";

static NORMAL_FRAGMENT_SRC: &str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif
varying vec3 ls_normal;

void main() {
    gl_FragColor = vec4((ls_normal + 1.0) / 2.0, 1.0);
}
";
