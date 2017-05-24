use std::io::BufReader;
use std::fs::File;

use serde_json;

use cgmath::*;
use shape::*;
use vec3d::*;
use light::*;

use color;
use color::Color;

use std;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    shapes: Vec<Shape>,
    light: Light,
    camera_pos: Vec3d,
    camera_dir: Vec3d,
    camera_up: Vec3d,
    max_reflection: i32,
}

impl Scene {
    pub fn new(light: Light, camera_pos: Vec3d, camera_dir: Vec3d, camera_up: Vec3d) -> Scene {
        Scene {
            shapes: Vec::new(),
            light: light,
            camera_pos: camera_pos,
            camera_dir: camera_dir,
            camera_up: camera_up,
            max_reflection: 4,
        }
    }

    pub fn from_file(filename: &str) -> Result<Scene, String> {
        let file_in = File::open(filename)
            .map_err(|e| format!("Can't open input file: {}", e))?;
        let reader = BufReader::new(&file_in);

        let s: Scene = serde_json::from_reader(reader)
            .map_err(|e| format!("JSON Parse error: {}", e))?;
        Ok(s)
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
        self.camera_dir = vec3_roty(self.camera_dir, x_rot);

        // project camera_dir to X plane
        let mut x_proj = self.camera_dir;
        x_proj.y = 0.0;
        x_proj = x_proj.normalize();

        let angle = if x_proj.x < 0.0 {
            -Vec3d::new(0.0, 0.0, 1.0).dot(x_proj).acos()
        } else {
            Vec3d::new(0.0, 0.0, 1.0).dot(x_proj).acos()
        };

        self.camera_dir = vec3_roty(self.camera_dir, -angle);
        self.camera_dir = vec3_rotx(self.camera_dir, y_rot);
        self.camera_dir = vec3_roty(self.camera_dir, angle);

        self.camera_dir = self.camera_dir.normalize();
    }


    /// Returns an iterator that scans a full frame at given resolution
    pub fn draw_iter(&self, h: usize, v: usize) -> DrawIter {
        DrawIter::new(self, h, v)
    }

    /// draw_iter converted to (u8, u8, u8)
    pub fn draw_iter_u8(&self,
                        h: usize,
                        v: usize)
                        -> std::iter::Map<DrawIter, fn(Color) -> (u8, u8, u8)> {
        self.draw_iter(h, v).map(Color::col_to_u8)
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
                        -> std::iter::Map<LineIter, fn(Color) -> (u8, u8, u8)> {
        self.line_iter(h, v, l).map(Color::col_to_u8)
    }

    // Recursively trace lines
    fn trace(&self, point: &Vec3d, dir: &Vec3d, depth: i32) -> Color {
        if depth >= self.max_reflection {
            return color::BLACK;
        }

        if let Some(intersect) = self.closest_q(point, dir) {
            let feeler_d = self.light.get_pos() - intersect.point;
            let dist_light = feeler_d.magnitude();
            let feeler_d_unit = feeler_d.normalize();
            let direct_light = self.is_direct_light(&intersect.point, &feeler_d_unit, dist_light);

            let local = phong(point, &intersect, &self.light, direct_light);

            let tmp = (intersect.point - point).normalize();
            let reflection_dir = tmp - (intersect.normal * 2.0 * tmp.dot(intersect.normal));

            let reflected = self.trace(&intersect.point, &reflection_dir, depth + 1);

            Color::add(&local,
                       &(Color::scale(&reflected, intersect.material.reflectivity)))
        } else {
            color::BLACK
        }
    }

    // Is there anything on the path to the light
    fn is_direct_light(&self, point: &Vec3d, dir: &Vec3d, dl: f64) -> bool {
        !self.shapes
             .iter()
             .map(|&ref x| x.intersect_dist(point, dir))
             .any(|dist_opt| dist_opt.map_or(false, |i| i < dl))
    }

    // Checks against all objects and returns closest intersection
    fn closest_q(&self, point: &Vec3d, dir: &Vec3d) -> Option<Intersection> {
        let find_min_opt = |min, (idx, op_val)| {
            match (min, op_val) {
                (Some((_, min_val)), Some(val)) if val < min_val => Some((idx, val)),
                (None, Some(val)) => Some((idx, val)),
                _ => min,
            }
        };

        let closest = self.shapes
            .iter()
            .map(|x| x.intersect_dist(point, dir))
            .enumerate()
            .fold(None, find_min_opt);

        // get intersection point info
        if let Some((idx, _)) = closest {
            self.shapes[idx].intersect(point, dir)
        } else {
            None
        }
    }
}

/// Iterator that iterates over a full frame
pub struct DrawIter<'a> {
    h_res: usize,
    v_res: usize,
    x: usize,
    y: usize,
    left: Vec3d,
    right_step: Vec3d,
    down_step: Vec3d,
    point: Vec3d,
    scene: &'a Scene,
}

impl<'a> DrawIter<'a> {
    fn new(scene: &Scene, h: usize, v: usize) -> DrawIter {
        let left = scene.camera_dir.cross(scene.camera_up).normalize();
        let up = left.cross(scene.camera_dir).normalize();
        let point = left + up + scene.camera_dir;
        let right_step = left * (-2.0 / h as f64);
        let down_step = up * (-2.0 / v as f64);
        DrawIter {
            h_res: h,
            v_res: v,
            x: 0,
            y: 0,
            left: left,
            point: point,
            right_step: right_step,
            down_step: down_step,
            scene: scene,
        }
    }
}

impl<'a> Iterator for DrawIter<'a> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.v_res {
            return None;
        }

        self.point = self.point + self.right_step;

        let c = self.scene
            .trace(&self.scene.camera_pos, &self.point.normalize(), 0);

        self.x += 1;
        if self.x >= self.h_res {
            self.x = 0;
            self.y += 1;
            self.point = self.point + self.left;
            self.point = self.point + self.left;
            self.point = self.point + self.down_step;
        }

        Some(c)
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
            .trace(&self.scene.camera_pos, &self.point.normalize(), 0);

        Some(c)
    }
}

// Calculate color with Phong model
fn phong(view_point: &Vec3d,
         intersection: &Intersection,
         light: &Light,
         direct_light: bool)
         -> Color {

    let point_material = intersection.material;
    let light_pos = light.get_pos();
    let light_color = light.get_color();
    let mut d = (light_pos - intersection.point)
        .normalize()
        .dot(intersection.normal);

    if d < 0.0 {
        d = 0.0;
    }

    let r_diffuse = if direct_light {
        light_color.0 * point_material.diffuse_color.0 * d
    } else {
        0.0
    };
    let g_diffuse = if direct_light {
        light_color.1 * point_material.diffuse_color.1 * d
    } else {
        0.0
    };
    let b_diffuse = if direct_light {
        light_color.2 * point_material.diffuse_color.2 * d
    } else {
        0.0
    };

    let r_ambient = point_material.ambient_color.0;
    let g_ambient = point_material.ambient_color.1;
    let b_ambient = point_material.ambient_color.2;

    let v = (view_point - intersection.point).normalize();
    let lt = (light_pos - intersection.point).normalize();
    let r = intersection.normal * (2.0 * intersection.normal.dot(lt)) - lt;

    let mut s = r.dot(v).powf(point_material.shininess);
    if s < 0.0 {
        s = 0.0;
    }

    let r_specular = if direct_light {
        light_color.0 * point_material.specular_color.0 * s
    } else {
        0.0
    };
    let g_specular = if direct_light {
        light_color.1 * point_material.specular_color.1 * s
    } else {
        0.0
    };
    let b_specular = if direct_light {
        light_color.2 * point_material.specular_color.2 * s
    } else {
        0.0
    };

    let mut r_sum = r_ambient + r_diffuse + r_specular;
    let mut g_sum = g_ambient + g_diffuse + g_specular;
    let mut b_sum = b_ambient + b_diffuse + b_specular;


    if r_sum > 1.0 {
        r_sum = 1.0;
    }

    if g_sum > 1.0 {
        g_sum = 1.0;
    }

    if b_sum > 1.0 {
        b_sum = 1.0;
    }

    Color(r_sum, g_sum, b_sum)
}
