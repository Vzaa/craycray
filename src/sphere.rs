use vecmath::*;
use vec3d::*;
use color::*;
use shape::*;
use material::*;

pub struct Sphere {
    center: Vec3d,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64, c: Color) -> Sphere {
        let material = Material {
            diffuse_color: c,
            ambient_color: BLACK,
            specular_color: WHITE,
            shininess: 15.0,
            reflectivity: 0.3,
        };
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }

    pub fn from_material(c: Vec3d, radius: f64, m: Material) -> Sphere {
        Sphere {
            center: c,
            radius: radius,
            material: m,
        }
    }
}

impl Shape for Sphere {
    fn intersect_dist(&self, p0: &Vec3d, d: &Vec3d) -> Option<f64> {
        let p0_min_c = vec3_sub(*p0, self.center);
        let a = vec3_dot(*d, *d);
        let b = 2.0 * vec3_dot(*d, p0_min_c);
        let c = vec3_dot(p0_min_c, p0_min_c) - (self.radius * self.radius);
        let delta = (b * b) - (4.0 * a * c);

        if delta < 0.0 {
            None
        } else {
            let delta_sq = delta.sqrt();
            let r0 = (-b - delta_sq) / (2.0 * a);
            let r1 = (-b + delta_sq) / (2.0 * a);
            if r0 > 0.5 && r0 < r1 {
                Some(r0)
            } else if r1 > 0.5 {
                Some(r1)
            } else {
                None
            }
        }
    }

    fn intersect(&self, p0: &Vec3d, d: &Vec3d) -> Option<Intersection> {
        let p0_min_c = vec3_sub(*p0, self.center);
        let a = vec3_dot(*d, *d);
        let b = 2.0 * vec3_dot(*d, p0_min_c);
        let c = vec3_dot(p0_min_c, p0_min_c) - (self.radius * self.radius);
        let delta = (b * b) - (4.0 * a * c);

        if delta < 0.0 {
            None
        } else {
            let delta_sq = delta.sqrt();
            let r0 = (-b - delta_sq) / (2.0 * a);
            let r1 = (-b + delta_sq) / (2.0 * a);
            if r0 > 0.5 && r0 < r1 {
                let q = vec3_add(*p0, vec3_scale(*d, r0));
                let n = vec3_normalized_sub(q, self.center);
                let mat = self.material;
                Some(Intersection {
                    material: mat,
                    point: q,
                    normal: n,
                })
            } else if r1 > 0.5 {
                let q = vec3_add(*p0, vec3_scale(*d, r1));
                let n = vec3_normalized_sub(q, self.center);
                let mat = self.material;
                Some(Intersection {
                    material: mat,
                    point: q,
                    normal: n,
                })
            } else {
                None
            }
        }
    }
}
