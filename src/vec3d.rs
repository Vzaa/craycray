use cgmath::Vector3;

pub type Vec3d = Vector3<f64>;

#[inline(always)]
pub fn vec3_rotx(a: Vec3d, angle: f64) -> Vec3d {
    let x = a.x;
    let y = (a.y * (angle).cos()) + (a.z * (-angle).sin());
    let z = (a.y * (angle).sin()) + (a.z * (angle).cos());

    Vec3d { x, y, z }
}

#[inline(always)]
pub fn vec3_roty(a: Vec3d, angle: f64) -> Vec3d {
    let x = (a.x * (angle).cos()) + (a.z * (angle).sin());
    let y = a.y;
    let z = (a.x * (-angle).sin()) + (a.z * (angle).cos());

    Vec3d { x, y, z }
}

#[inline(always)]
pub fn vec3_rotz(a: Vec3d, angle: f64) -> Vec3d {
    let x = (a.x * (angle).cos()) + (a.y * (-angle).sin());
    let y = (a.x * (angle).sin()) + (a.y * (angle).cos());
    let z = a.z;

    Vec3d { x, y, z }
}
