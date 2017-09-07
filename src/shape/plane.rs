use cgmath::*;
use shape::*;
use color;

#[derive(Serialize, Deserialize)]
pub struct Plane {
    material: Material,
    point: Vec3d,
    normal: Vec3d,
}

impl Plane {
    pub fn new(point: Vec3d, normal: Vec3d, c: Color) -> Plane {
        let material = Material {
            diffuse_color: c,
            ambient_color: color::BLACK,
            specular_color: color::BLACK,
            shininess: 15.0,
            reflectivity: 0.1,
        };
        Plane {
            point: point,
            normal: normal.normalize(),
            material: material,
        }
    }

    pub fn from_material(point: Vec3d, normal: Vec3d, material: Material) -> Plane {
        Plane {
            point: point,
            normal: normal.normalize(),
            material: material,
        }
    }
}

impl Intersectable for Plane {
    fn intersect_dist(&self, p0: Vec3d, d: Vec3d) -> Option<f64> {
        let neg_norm = self.normal * -1.0;
        let denom = neg_norm.dot(d);
        if denom > 1e-6 {
            let p0l0 = self.point - p0;
            let t = (p0l0.dot(neg_norm)) / denom;
            if t >= 0.0 {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn intersect(&self, p0: Vec3d, d: Vec3d) -> Option<Intersection> {
        let neg_norm = self.normal * -1.0;
        let denom = neg_norm.dot(d);
        if denom > 1e-6 {
            let p0l0 = self.point - p0;
            let t = (p0l0.dot(neg_norm)) / denom;
            if t >= 0.0 {
                let dir_scaled = d * t;
                let q = p0 + dir_scaled;

                Some(Intersection {
                    material: &self.material,
                    point: q,
                    normal: self.normal,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
