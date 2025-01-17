use probe::{class::Instance, JavaClass};

use crate::java::mc::util::math::{BlockPos, Direction, Vec3d};

#[derive(JavaClass)]
#[package(net.minecraft.entity.player)]
pub struct PlayerEntity {
    #[instance]
    pub raw: Instance,
}

#[probe::import]
impl PlayerEntity {
    pub fn get_block_pos(self) -> BlockPos;
    pub fn get_facing(self) -> Direction;
    pub fn get_rotation_vector(self) -> Vec3d;
}

// new_class! {PlayerEntity: "net/minecraft/entity/player/PlayerEntity" {
//     pub getBlockPos fn get_block_pos() -> BlockPos;
//     pub getFacing fn get_facing() -> Direction;
//     pub getRotationVector fn get_rotation_vector() -> Vec3d;
//     // pub getHorizontalFacing fn get_horizontal_facing() -> Direction<'local>;
// }}
