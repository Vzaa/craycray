use vecmath::*;

use shape::*;
use vec3d::*;
use light::*;
use color::*;

use std;

pub struct Scene {
    shapes: Vec<Box<Shape + Sync>>,
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

    pub fn add_shape(&mut self, s: Box<Shape + Sync>) {
        self.shapes.push(s);
    }

    pub fn step(&mut self) {
        //self.light.translate(&[1.0, 0.0, 0.0]);
    }

    pub fn mv_camera_fwd(&mut self) {
        self.camera_pos = vec3_add(self.camera_pos, self.camera_dir);
    }

    pub fn mv_camera_back(&mut self) {
        self.camera_pos = vec3_sub(self.camera_pos, self.camera_dir);
    }

    pub fn rot_camera(&mut self, x_rot: f64, y_rot: f64) {
        self.camera_dir = vec3_roty(self.camera_dir, x_rot);

        // project camera_dir to X plane
        let mut x_proj = self.camera_dir.clone();
        x_proj[1] = 0.0;
        x_proj = vec3_normalized(x_proj);

        let angle = if x_proj[0] < 0.0 {
            -vec3_dot([0.0, 0.0, 1.0], x_proj).acos()
        } else {
            vec3_dot([0.0, 0.0, 1.0], x_proj).acos()
        };

        self.camera_dir = vec3_roty(self.camera_dir, -angle);
        self.camera_dir = vec3_rotx(self.camera_dir, y_rot);
        self.camera_dir = vec3_roty(self.camera_dir, angle);

        self.camera_dir = vec3_normalized(self.camera_dir);
    }


    /// Returns an iterator that scans a full frame at given resolution
    pub fn draw_iter(&self, h: usize, v: usize) -> DrawIter {
        DrawIter::new(&self, h, v)
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
        LineIter::new(&self, h, v, l)
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
            return BLACK;
        }

        let intersect_res = self.closest_q(point, dir);
        if let Some(intersect) = intersect_res {
            let feeler_d = vec3_sub(*self.light.get_pos(), intersect.point);
            let dist_light = vec3_len(feeler_d);
            let feeler_d_unit = vec3_normalized(feeler_d);
            let direct_light =
                self.is_direct_light(&intersect.point, &feeler_d_unit, dist_light);

            let local = phong(point, &intersect, &self.light, direct_light);

            let tmp = vec3_normalized_sub(intersect.point, *point);
            let reflection_dir =
                vec3_normalized_sub(tmp,
                                    vec3_scale(intersect.normal,
                                               2.0 * vec3_dot(tmp, intersect.normal)));

            let reflected = self.trace(&intersect.point, &reflection_dir, depth + 1);

            Color::add(&local, &(Color::scale(&reflected, intersect.material.reflectivity)))
        } else {
            BLACK
        }
    }

    // Is there anything on the path to the light
    fn is_direct_light(&self, point: &Vec3d, dir: &Vec3d, dl: f64) -> bool {
        let unitd = vec3_normalized(*dir);
        !self.shapes
            .iter()
            .map(|&ref x| x.intersect_dist(point, &unitd))
            .any(|dist_opt| dist_opt.map(|i| i < dl).unwrap_or(false))
    }

    // Checks against all objects and returns closest intersection
    fn closest_q(&self, point: &Vec3d, dir: &Vec3d) -> Option<Intersection> {
        let unitd = vec3_normalized(*dir);

        let closest = self.shapes
            .iter()
            .map(|&ref x| x.intersect_dist(point, &unitd))
            .zip(&self.shapes)
            .fold(None, |min, (dist_op, shape)| {
                match (dist_op, min) {
                    (Some(dist_val), Some((dist_min, _))) if dist_val < dist_min => {
                        Some((dist_val, shape))
                    }
                    (Some(dist_val), None) => Some((dist_val, shape)),
                    _ => min,
                }
            });

        // get intersection point info
        if let Some((_, shape)) = closest {
            shape.intersect(&point, &dir)
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
        let left = vec3_normalized(vec3_cross(scene.camera_dir, scene.camera_up));
        let up = vec3_normalized(vec3_cross(left, scene.camera_dir));
        let point = vec3_add(vec3_add(left, up), scene.camera_dir);
        let right_step = vec3_scale(left, -2.0 / h as f64);
        let down_step = vec3_scale(up, -2.0 / v as f64);
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

        self.point = vec3_add(self.point, self.right_step);

        let c = self.scene.trace(&self.scene.camera_pos, &vec3_normalized(self.point), 0);

        self.x += 1;
        if self.x >= self.h_res {
            self.x = 0;
            self.y += 1;
            self.point = vec3_add(self.point, self.left);
            self.point = vec3_add(self.point, self.left);
            self.point = vec3_add(self.point, self.down_step);
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
        let left = vec3_normalized(vec3_cross(scene.camera_dir, scene.camera_up));
        let up = vec3_normalized(vec3_cross(left, scene.camera_dir));
        let mut point = vec3_add(vec3_add(left, up), scene.camera_dir);
        let right_step = vec3_scale(left, -2.0 / h as f64);
        let down_step = vec3_scale(up, -2.0 / v as f64);

        point = vec3_add(point, vec3_scale(down_step, l as f64));
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

        self.point = vec3_add(self.point, self.right_step);

        let c = self.scene.trace(&self.scene.camera_pos, &vec3_normalized(self.point), 0);

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
    let mut d = vec3_dot(vec3_normalized_sub(*light_pos, intersection.point),
                         intersection.normal);

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

    let v = vec3_normalized_sub(*view_point, intersection.point);
    let lt = vec3_normalized_sub(*light_pos, intersection.point);
    let r = vec3_normalized_sub(vec3_scale(intersection.normal,
                                           2.0 * vec3_dot(intersection.normal, lt)),
                                lt);

    let mut s = vec3_dot(r, v).powf(point_material.shininess);
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
