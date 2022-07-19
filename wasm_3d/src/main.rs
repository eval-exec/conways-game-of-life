use std::cell::RefCell;
use std::f64::consts::{FRAC_1_PI, FRAC_2_PI, FRAC_PI_2, PI};
use std::rc::Rc;

use fastrand::{f32, f64, u32, usize};
use kiss3d::camera::FirstPerson;
use kiss3d::light::Light;
use kiss3d::ncollide3d::na;
use kiss3d::ncollide3d::procedural::{IndexBuffer, TriMesh, utils};
use kiss3d::ncollide3d::simba::scalar::RealField;
use kiss3d::resource::Mesh;
use kiss3d::scene::{PlanarSceneNode, SceneNode};
use kiss3d::window::Window;
use nalgebra::{
    Isometry2, Isometry3, Point2, Point3, Translation, Translation3, U24, UnitQuaternion, Vector3,
};

use crate::color::color::{BLACK, WHITE, YELLOW};
use crate::life::Live;

mod ascii;
mod color;
mod life;

// Black	#000000	(0,0,0)
// White	#FFFFFF	(255,255,255)
// Red	#FF0000	(255,0,0)
// Lime	#00FF00	(0,255,0)
// Blue	#0000FF	(0,0,255)
// Yellow	#FFFF00	(255,255,0)
// Cyan	#00FFFF	(0,255,255)
// Magenta	#FF00FF	(255,0,255)
// Silver	#C0C0C0	(192,192,192)
// Gray	#808080	(128,128,128)
// Maroon	#800000	(128,0,0)
// Olive	#808000	(128,128,0)
// Green	#008000	(0,128,0)
// Purple	#800080	(128,0,128)
// Teal	#008080	(0,128,128)
// Navy	#000080	(0,0,128)
//

/// Generates a cylinder with unit height and diameter.
pub fn xunit_cylinder<N: RealField + Copy>(nsubdiv: u32, grid_width: u32, r: u32) -> TriMesh<N> {
    let angle = N::from_f64(grid_width as f64 / r as f64).unwrap();
    let invsubdiv = nalgebra::one::<N>() / nalgebra::convert(nsubdiv as f64);
    let dtheta = angle * invsubdiv;
    let mut coords = Vec::new();
    let mut indices = Vec::new();
    let mut normals: Vec<Vector3<N>>;

    utils::push_circle(
        na::convert(0.5),
        nsubdiv,
        dtheta,
        na::convert(-0.5),
        &mut coords,
    );

    normals = coords.iter().map(|p| p.coords).collect();

    utils::push_circle(
        na::convert(0.5),
        nsubdiv,
        dtheta,
        na::convert(0.5),
        &mut coords,
    );

    utils::push_ring_indices(0, nsubdiv, nsubdiv, &mut indices);
    utils::push_filled_circle_indices(0, nsubdiv, &mut indices);
    utils::push_filled_circle_indices(nsubdiv, nsubdiv, &mut indices);

    let len = indices.len();
    let bottom_start_id = len - (nsubdiv as usize - 2);
    utils::reverse_clockwising(&mut indices[bottom_start_id..]);

    let mut indices = utils::split_index_buffer(&indices[..]);

    /*
     * Compute uvs.
     */
    // bottom ring uvs
    let mut uvs = Vec::with_capacity(coords.len());
    let mut curr_u = N::zero();
    for _ in 0..nsubdiv {
        uvs.push(Point2::new(curr_u, na::zero()));
        curr_u = curr_u + invsubdiv;
    }

    // top ring uvs
    curr_u = na::zero();
    for _ in 0..nsubdiv {
        uvs.push(Point2::new(curr_u, na::one()));
        curr_u = curr_u + invsubdiv;
    }

    /*
     * Adjust normals.
     */
    for n in normals.iter_mut() {
        n.x = n.x * na::convert(2.0);
        n.y = na::zero();
        n.z = n.z * na::convert(2.0);
    }

    normals.push(Vector3::y()); // top cap
    normals.push(-Vector3::y()); // bottom cap
    let nlen = normals.len() as u32;

    let top_start_id = len - 2 * (nsubdiv as usize - 2);

    for i in indices[..top_start_id].iter_mut() {
        if i.x.y >= nsubdiv {
            i.x.y = i.x.y - nsubdiv;
        }
        if i.y.y >= nsubdiv {
            i.y.y = i.y.y - nsubdiv;
        }
        if i.z.y >= nsubdiv {
            i.z.y = i.z.y - nsubdiv;
        }
    }

    for i in indices[top_start_id..bottom_start_id].iter_mut() {
        i.x.y = nlen - 2;
        i.y.y = nlen - 2;
        i.z.y = nlen - 2;
    }

    for i in indices[bottom_start_id..].iter_mut() {
        i.x.y = nlen - 1;
        i.y.y = nlen - 1;
        i.z.y = nlen - 1;
    }

    TriMesh::new(
        coords,
        Some(normals),
        Some(uvs),
        Some(IndexBuffer::Split(indices)),
    )
}

fn set_color(sc: &mut SceneNode, c: rgb::RGB8) {
    sc.set_color(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0)
}

fn rand_color() -> rgb::RGB8 {
    rgb::RGB8::new(
        fastrand::u8(0..=255),
        fastrand::u8(0..=255),
        fastrand::u8(0..=255),
    )
}

fn main() {
    let mut window = Window::new("Happy Birthday! @EXEC!" );
    window.set_light(Light::StickToCamera);

    let radious: f32 = 90.0;
    let grid_width = 3;
    let text_grid_width = 1;
    let tri_mesh = xunit_cylinder(500, grid_width, radious as u32);

    let mesh = Rc::new(RefCell::new(Mesh::from_trimesh(tri_mesh, false)));

    // let height = (radious as f64 / grid_width as f64) as usize;
    let height = (radious * 2.0 * 0.5 / grid_width as f32) as usize;

    let width = (PI * 2.0 * radious as f64 / grid_width as f64) as usize;

    let mut nodes: Vec<SceneNode> = Vec::new();
    let scale = Vector3::new(
        radious as f32 * 2.0,
        grid_width as f32,
        radious as f32 * 2.0,
    );
    for level in 0..=height {
        for i in 0..=width {
            let mut mesh_cylinder = window.add_mesh(mesh.clone(), scale);
            set_color(&mut mesh_cylinder, rand_color());
            mesh_cylinder.set_local_translation(Translation3::new(
                0.0,
                level as f32 * grid_width as f32,
                0.0,
            ));
            // mesh_cylinder.set_local_transformation(Isometry3::new(
            //     Vector3::new(0.0, level as f32 * grid_width as f32 * 1.1, 0.0),
            //     Vector3::new(0.0, 0.0, 0.0),
            // ));
            let rot = UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                i as f32 * grid_width as f32 / radious as f32,
            );
            mesh_cylinder.set_local_rotation(rot);
            nodes.push(mesh_cylinder);
        }
    }

    let mut universe = life::Universe::new(width + 1, height + 1);

    let mut text_body = window.add_group();
    let mut display_text = |st: &str, linenum: u32| {
        let char_vec: Vec<char> = st.chars().collect();

        let mut text_line = text_body.add_group();
        for (idxCh, ch) in char_vec.iter().enumerate() {
            let mut one_char = text_line.add_group();
            let vc = ascii::ascii_map::get(*ch);
            let vc2 = vc
                .iter()
                .flat_map(|v| -> Vec<u8> {
                    let tail = v & 0x0F;
                    let head = v >> 4;
                    vec![head, tail]
                })
                .collect::<Vec<u8>>();
            for line in 0..16 {
                let mut first: u32 = vc2[line * 3 + 0] as u32;
                first = first << 8;
                let mut second: u32 = vc2[line * 3 + 1] as u32;
                second = second << 4;
                let mut third: u32 = vc2[line * 3 + 2] as u32;
                let row = first + second + third;

                for i in 1..=12 {
                    if (1 << (12 - i + 1)) & row > 0 {
                        let mut cube =
                            one_char.add_cube(text_grid_width as f32, text_grid_width as f32, text_grid_width as f32);
                        cube.set_local_translation(Translation3::new(
                            text_grid_width as f32 * i as f32,
                            0.0,
                            text_grid_width as f32 * line as f32,
                        ))
                    }
                }
            }
            one_char.set_local_translation(Translation3::new(
                (idxCh as u32 * (text_grid_width * 13)) as f32,
                height as f32 * grid_width as f32,
                0.0 + (linenum * 25 * text_grid_width) as f32,
            ));
        }
    };
    display_text("Happy", 0);
    display_text("Birthday", 1);
    display_text("@EXEC", 2);
    text_body.set_local_translation(Translation3::new(
        0.0 - (text_grid_width * 16 * 3) as f32,
        0.0,
        0.0 - (text_grid_width * 25) as f32,
    ));
    // nodes.push(text_body);

    // let mut base_cylinder = window.add_cylinder(1.0, 0.5);
    // let mut second_cylinder = window.add_cylinder(0.8, 0.5);

    // base_cylinder.set_color(1.0, 0.0, 0.0);
    // second_cylinder.set_color(230.0 / 255.0, 209.0 / 255.0, 202.0 / 255.0);

    let eye_target_ratio = 3.0;
    let eye = Point3::new(
        radious * eye_target_ratio/2.0,
        radious * eye_target_ratio,
        radious * eye_target_ratio/2.0,
    );
    let at = Point3::new(0.0, (height as u32 * grid_width) as f32, 0.0);
    let mut first_person = FirstPerson::new(eye, at);


    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.01);


    while window.render_with_camera(&mut first_person) {
        // let zero = Point3::new(0.0, 0.0, 0.0);
        // let xmax = Point3::new(100.0, 0.0, 0.0);
        // let ymax = Point3::new(0.0, 100.0, 0.0);
        // let zmax = Point3::new(0.0, 0.0, 100.0);
        //
        // window.set_line_width(2.0);
        // window.draw_line(&zero, &xmax, &Point3::new(1.0, 0.0, 0.0));
        // window.draw_line(&zero, &ymax, &Point3::new(0.0, 1.0, 0.0));
        // window.draw_line(&zero, &zmax, &Point3::new(0.0, 0.0, 1.0));

        // top.prepend_to_local_rotation(&rot);
        let mut pk = |h: usize, w: usize, live: Live| {
            if live.eq(&Live::Alive) {
                set_color(&mut nodes[w + h * (width + 1)], rand_color());
            } else {
                set_color(&mut nodes[w + h * (width + 1)], BLACK);
            }
        };
        universe.tick(&mut pk);

        for node in &mut nodes {
            node.prepend_to_local_rotation(&rot);
        }
    }
}
