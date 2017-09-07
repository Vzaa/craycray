use cgmath::*;
use shape::*;
use color;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    material: Material,
    center: Vec3d,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64, c: Color) -> Sphere {
        let material = Material {
            diffuse_color: c,
            ambient_color: color::BLACK,
            specular_color: color::WHITE,
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

impl Intersectable for Sphere {
    fn intersect_dist(&self, p0: Vec3d, d: Vec3d) -> Option<f64> {
        let p0_min_c = p0 - self.center;
        let a = d.dot(d);
        let b = 2.0 * d.dot(p0_min_c);
        let c = p0_min_c.dot(p0_min_c) - (self.radius * self.radius);
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

    fn intersect(&self, p0: Vec3d, d: Vec3d) -> Option<Intersection> {
        let p0_min_c = p0 - self.center;
        let a = d.dot(d);
        let b = 2.0 * d.dot(p0_min_c);
        let c = p0_min_c.dot(p0_min_c) - (self.radius * self.radius);
        let delta = (b * b) - (4.0 * a * c);

        if delta < 0.0 {
            None
        } else {
            let delta_sq = delta.sqrt();
            let r0 = (-b - delta_sq) / (2.0 * a);
            let r1 = (-b + delta_sq) / (2.0 * a);
            if r0 > 0.5 && r0 < r1 {
                let q = p0 + d * r0;
                let n = (q - self.center).normalize();
                Some(Intersection {
                    material: &self.material,
                    point: q,
                    normal: n,
                })
            } else if r1 > 0.5 {
                let q = p0 + d * r1;
                let n = (q - self.center).normalize();
                Some(Intersection {
                    material: &self.material,
                    point: q,
                    normal: n,
                })
            } else {
                None
            }
        }
    }
}
