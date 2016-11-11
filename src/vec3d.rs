use vecmath::Vector3;
pub type Vec3d = Vector3<f64>;

#[inline(always)]
pub fn vec3_rotx(a: Vec3d, angle: f64) -> Vec3d {
    [a[0],
     (a[1] * (angle).cos()) + (a[2] * (-angle).sin()),
     (a[1] * (angle).sin()) + (a[2] * (angle).cos())]
}

#[inline(always)]
pub fn vec3_roty(a: Vec3d, angle: f64) -> Vec3d {
    [(a[0] * (angle).cos()) + (a[2] * (angle).sin()),
     a[1],
     (a[0] * (-angle).sin()) + (a[2] * (angle).cos())]
}

#[inline(always)]
pub fn vec3_rotz(a: Vec3d, angle: f64) -> Vec3d {
    [(a[0] * (angle).cos()) + (a[1] * (-angle).sin()),
     (a[0] * (angle).sin()) + (a[1] * (angle).cos()),
     a[2]]
}
