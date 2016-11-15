use color::*;

#[derive(Copy, Clone)]
pub struct Material {
    pub ambient_color: Color,
    pub specular_color: Color,
    pub diffuse_color: Color,
    pub shininess: f64,
    pub reflectivity: f64,
}

pub const MIRROR: Material = Material {
    ambient_color: BLACK,
    specular_color: BLACK,
    diffuse_color: BLACK,
    shininess: 0.0,
    reflectivity: 1.0,
};
