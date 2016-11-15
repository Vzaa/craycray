use vecmath::*;
use vec3d::*;
use color::*;
use material::*;
use shape::*;

// TODO: This is incomplete

pub struct Plane {
    point: Vec3d,
    normal: Vec3d,
    material: Material,
}

impl Plane {
    pub fn new(point: Vec3d, normal: Vec3d, c: Color) -> Plane {
        let material = Material {
            diffuse_color: c,
            ambient_color: BLACK,
            specular_color: WHITE,
            shininess: 15.0,
            reflectivity: 0.3,
        };
        Plane {
            point: point,
            normal: vec3_normalized(normal),
            material: material,
        }
    }

    pub fn from_material(point: Vec3d, normal: Vec3d, material: Material) -> Plane {
        Plane {
            point: point,
            normal: vec3_normalized(normal),
            material: material,
        }
    }
}

impl Shape for Plane {
    fn intersect_dist(&self, p0: &Vec3d, d: &Vec3d) -> Option<f64> {
        let denom = vec3_dot(self.normal, *d);
        if denom > 1e-6 {
            let p0l0 = vec3_sub(self.point, *p0);
            let t = vec3_dot(p0l0, self.normal) / denom;
            if t >= 0.0 { Some(t) } else { None }
        } else {
            None
        }
    }

    fn intersect(&self, p0: &Vec3d, d: &Vec3d) -> Option<Intersection> {
        let denom = vec3_dot(self.normal, *d);
        if denom > 1e-6 {
            let p0l0 = vec3_sub(self.point, *p0);
            let t = vec3_dot(p0l0, self.normal) / denom;
            if t >= 0.0 {
                let q = vec3_scale(vec3_add(*p0, *d), t);

                Some(Intersection {
                    material: self.material,
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
