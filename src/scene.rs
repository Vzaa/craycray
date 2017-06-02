use std::io;
use std::iter::Map;
use std::io::BufReader;
use std::fs::File;

use serde_json;

use cgmath::*;
use shape::*;
use vec3d::Vec3d;
use vec3d::Rotatable;
use light::Light;

use color;
use color::Color;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    shapes: Vec<Shape>,
    lights: Vec<Light>,
    camera_pos: Vec3d,
    camera_dir: Vec3d,
    camera_up: Vec3d,
    max_reflection: i32,
}

#[derive(Debug)]
pub enum CraycrayError {
    Io(io::Error),
    Serde(serde_json::Error),
}

impl Scene {
    pub fn new(camera_pos: Vec3d, camera_dir: Vec3d, camera_up: Vec3d) -> Scene {
        Scene {
            shapes: Vec::new(),
            lights: Vec::new(),
            camera_pos: camera_pos,
            camera_dir: camera_dir,
            camera_up: camera_up,
            max_reflection: 4,
        }
    }

    pub fn from_file(filename: &str) -> Result<Scene, CraycrayError> {
        File::open(filename)
            .map_err(CraycrayError::Io)
            .and_then(|f| serde_json::from_reader(BufReader::new(f)).map_err(CraycrayError::Serde))
    }

    pub fn add_shape(&mut self, s: Shape) {
        self.shapes.push(s);
    }

    pub fn step(&mut self) {
        // self.light.translate(&[1.0, 0.0, 0.0]);
    }

    pub fn mv_camera_fwd(&mut self) {
        self.camera_pos = self.camera_pos + self.camera_dir;
    }

    pub fn mv_camera_back(&mut self) {
        self.camera_pos = self.camera_pos - self.camera_dir;
    }

    pub fn rot_camera(&mut self, x_rot: f64, y_rot: f64) {
        // project camera_dir to X plane
        let mut x_proj = self.camera_dir;
        x_proj.y = 0.0;
        x_proj = x_proj.normalize();

        let angle = if x_proj.x < 0.0 {
            -Vec3d::new(0.0, 0.0, 1.0).dot(x_proj).acos()
        } else {
            Vec3d::new(0.0, 0.0, 1.0).dot(x_proj).acos()
        };

        self.camera_dir = self.camera_dir
            .rot_y(x_rot)
            .rot_y(-angle)
            .rot_x(y_rot)
            .rot_y(angle)
            .normalize();
    }

    /// Returns an iterator for a line at given resolution
    pub fn line_iter(&self, h: usize, v: usize, l: usize) -> LineIter {
        LineIter::new(self, h, v, l)
    }

    /// line_iter converted to (u8, u8, u8)
    pub fn line_iter_u8(&self,
                        h: usize,
                        v: usize,
                        l: usize)
                        -> Map<LineIter, fn(Color) -> (u8, u8, u8)> {
        self.line_iter(h, v, l).map(Color::col_to_u8)
    }

    // Recursively trace lines
    fn trace(&self, point: Vec3d, dir: Vec3d, depth: i32) -> Color {
        if depth >= self.max_reflection {
            return color::BLACK;
        }

        if let Some(intersect) = self.closest_q(point, dir) {
            let local = self.lights
                .iter()
                .filter(|l| {
                         let (f_unit, dist) = l.feeler(intersect.point);
                         self.is_direct_light(intersect.point, f_unit, dist)
                     })
                .map(|l| phong(point, &intersect, l))
                .sum::<Color>() + intersect.material.ambient_color;

            let tmp = (intersect.point - point).normalize();
            let reflection_dir = tmp - (intersect.normal * 2.0 * tmp.dot(intersect.normal));

            let reflected = self.trace(intersect.point, reflection_dir, depth + 1);

            local + (reflected * intersect.material.reflectivity)
        } else {
            color::BLACK
        }
    }

    // Is there anything on the path to the light
    fn is_direct_light(&self, point: Vec3d, dir: Vec3d, dl: f64) -> bool {
        !self.shapes
             .iter()
             .map(|&ref x| x.intersect_dist(point, dir))
             .any(|dist_opt| dist_opt.map_or(false, |i| i < dl))
    }

    // Checks against all objects and returns closest intersection
    fn closest_q(&self, point: Vec3d, dir: Vec3d) -> Option<Intersection> {
        let find_min_opt = |min, (idx, op_val)| {
            match (min, op_val) {
                (Some((_, min_val)), Some(val)) if val < min_val => Some((idx, val)),
                (None, Some(val)) => Some((idx, val)),
                _ => min,
            }
        };

        self.shapes
            .iter()
            .map(|x| x.intersect_dist(point, dir))
            .enumerate()
            .fold(None, find_min_opt)
            .and_then(|(idx, _)| self.shapes[idx].intersect(point, dir))
    }
}

/// Iterator that iterates over a single line
pub struct LineIter<'a> {
    h_res: usize,
    x: usize,
    right_step: Vec3d,
    point: Vec3d,
    scene: &'a Scene,
}

impl<'a> LineIter<'a> {
    fn new(scene: &Scene, h: usize, v: usize, l: usize) -> LineIter {
        let left = scene.camera_dir.cross(scene.camera_up).normalize();
        let up = left.cross(scene.camera_dir).normalize();
        let mut point = left + up + scene.camera_dir;
        let right_step = left * (-2.0 / h as f64);
        let down_step = up * (-2.0 / v as f64);

        point = point + down_step * (l as f64);
        LineIter {
            h_res: h,
            x: 0,
            point: point,
            right_step: right_step,
            scene: scene,
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.h_res {
            return None;
        }
        self.x += 1;

        self.point = self.point + self.right_step;

        let c = self.scene
            .trace(self.scene.camera_pos, self.point.normalize(), 0);

        Some(c)
    }
}

// Calculate color with Phong model
fn phong(view_point: Vec3d, intersection: &Intersection, light: &Light) -> Color {
    let point_material = intersection.material;
    let light_pos = light.get_pos();
    let light_color = light.get_color();

    let mut d = (light_pos - intersection.point)
        .normalize()
        .dot(intersection.normal);

    if d < 0.0 {
        d = 0.0;
    }

    let v = (view_point - intersection.point).normalize();
    let lt = (light_pos - intersection.point).normalize();
    let r = intersection.normal * (2.0 * intersection.normal.dot(lt)) - lt;

    let mut s = r.dot(v).powf(point_material.shininess);
    if s < 0.0 {
        s = 0.0;
    }

    let diffuse = light_color * point_material.diffuse_color * d;
    let specular = light_color * point_material.specular_color * s;

    diffuse + specular
}
