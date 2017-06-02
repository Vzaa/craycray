use vec3d::Vec3d;
use color::Color;
use cgmath::*;

#[derive(Serialize, Deserialize)]
pub struct Light {
    pos: Vec3d,
    color: Color,
}

impl Light {
    pub fn new(v: Vec3d, color: Color) -> Light {
        Light {
            pos: v,
            color: color,
        }
    }

    pub fn get_pos(&self) -> Vec3d {
        self.pos
    }

    pub fn translate(&mut self, v: &Vec3d) {
        self.pos = v + self.pos;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn feeler(&self, point: Vec3d) -> (Vec3d, f64) {
        let feeler_d = self.pos - point;
        (feeler_d.normalize(), feeler_d.magnitude())
    }
}
