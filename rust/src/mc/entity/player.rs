use crate::{
    mc::util::math::{BlockPos, Direction, Vec3d},
    new_class,
};

new_class! {PlayerEntity: "net/minecraft/entity/player/PlayerEntity" {
    pub getBlockPos fn get_block_pos() -> BlockPos;
    pub getFacing fn get_facing() -> Direction;
    pub getRotationVector fn get_rotation_vector() -> Vec3d;
    // pub getHorizontalFacing fn get_horizontal_facing() -> Direction<'local>;
}}
