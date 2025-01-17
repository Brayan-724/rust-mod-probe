// use crate::new_class;

use probe::{class::Instance, JavaClass};

#[derive(JavaClass)]
#[package(net.minecraft.util.math)]
pub struct BlockPos {
    #[instance]
    pub raw: Instance,
}

#[probe::import]
impl BlockPos {
    pub fn offset(self, direction: Direction, distance: i32);
    pub fn to_center_pos(self) -> Vec3d;
}

#[derive(JavaClass)]
#[package(net.minecraft.util.math)]
pub struct Direction {
    #[instance]
    pub raw: Instance,
}

#[derive(JavaClass)]
#[package(net.minecraft.util.math)]
pub struct Vec3d {
    #[instance]
    pub raw: Instance,
}

#[probe::import]
impl Vec3d {
    pub extern "multiply" fn scale(self, scale: f64) -> Self;
    pub extern "add" fn add_vec(self, vec: Self) -> Self;
}
