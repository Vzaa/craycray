use vec3d::*;
use color::*;
use material::*;

pub struct Intersection {
    pub material: Material, // This could be a reference?
    pub point: Vec3d,
    pub normal: Vec3d,
}

pub trait Shape {
    fn intersect_dist(&self, p0: &Vec3d, d: &Vec3d) -> Option<f64>;
    fn intersect(&self, p0: &Vec3d, d: &Vec3d) -> Option<Intersection>;
}
