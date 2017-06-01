pub mod sphere;
pub mod plane;

use vec3d::Vec3d;
use material::Material;
use self::sphere::Sphere;
use self::plane::Plane;
use color::Color;

pub struct Intersection {
    pub material: Material, // This could be a reference?
    pub point: Vec3d,
    pub normal: Vec3d,
}

pub trait Intersectable {
    fn intersect_dist(&self, p0: Vec3d, d: Vec3d) -> Option<f64>;
    fn intersect(&self, p0: Vec3d, d: Vec3d) -> Option<Intersection>;
}

#[derive(Serialize, Deserialize)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn new_sphere(center: Vec3d, radius: f64, c: Color) -> Shape {
        Shape::Sphere(Sphere::new(center, radius, c))
    }

    pub fn new_sphere_material(center: Vec3d, radius: f64, m: Material) -> Shape {
        Shape::Sphere(Sphere::from_material(center, radius, m))
    }

    pub fn new_plane(point: Vec3d, normal: Vec3d, c: Color) -> Shape {
        Shape::Plane(Plane::new(point, normal, c))
    }

    pub fn new_plane_material(point: Vec3d, normal: Vec3d, m: Material) -> Shape {
        Shape::Plane(Plane::from_material(point, normal, m))
    }
}

impl Intersectable for Shape {
    fn intersect_dist(&self, p0: Vec3d, d: Vec3d) -> Option<f64> {
        match *self {
            Shape::Sphere(ref s) => s.intersect_dist(p0, d),
            Shape::Plane(ref p) => p.intersect_dist(p0, d),
        }
    }

    fn intersect(&self, p0: Vec3d, d: Vec3d) -> Option<Intersection> {
        match *self {
            Shape::Sphere(ref s) => s.intersect(p0, d),
            Shape::Plane(ref p) => p.intersect(p0, d),
        }
    }
}
