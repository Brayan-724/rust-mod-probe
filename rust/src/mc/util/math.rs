use crate::new_class;

new_class! {BlockPos: "net/minecraft/util/math/BlockPos" {
    pub fn offset(direction: Direction, distance: i32) -> BlockPos;
    pub toCenterPos fn to_center_pos() -> Vec3d;
}}

new_class! {Direction: "net/minecraft/util/math/Direction" {}}

new_class! {Vec3d: "net/minecraft/util/math/Vec3d" {
    pub fn multiply(scale: f64) -> Vec3d;
    pub fn add(vec: Vec3d) -> Vec3d;
}}
