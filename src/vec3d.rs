use cgmath::Vector3;

pub type Vec3d = Vector3<f64>;

pub trait Rotatable {
    fn rot_x(self, angle: f64) -> Vec3d;
    fn rot_y(self, angle: f64) -> Vec3d;
    fn rot_z(self, angle: f64) -> Vec3d;
}

impl Rotatable for Vec3d {
    fn rot_x(self, angle: f64) -> Vec3d {
        let x = self.x;
        let y = (self.y * (angle).cos()) + (self.z * (-angle).sin());
        let z = (self.y * (angle).sin()) + (self.z * (angle).cos());

        Vec3d { x, y, z }
    }

    fn rot_y(self, angle: f64) -> Vec3d {
        let x = (self.x * (angle).cos()) + (self.z * (angle).sin());
        let y = self.y;
        let z = (self.x * (-angle).sin()) + (self.z * (angle).cos());

        Vec3d { x, y, z }
    }

    fn rot_z(self, angle: f64) -> Vec3d {
        let x = (self.x * (angle).cos()) + (self.y * (-angle).sin());
        let y = (self.x * (angle).sin()) + (self.y * (angle).cos());
        let z = self.z;

        Vec3d { x, y, z }
    }
}
