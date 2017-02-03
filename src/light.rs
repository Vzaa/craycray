use vec3d::Vec3d;
use color::Color;
use vecmath::*;

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

    pub fn get_pos(&self) -> &Vec3d {
        &self.pos
    }

    pub fn translate(&mut self, v: &Vec3d) {
        self.pos = vec3_add(*v, self.pos);
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}
