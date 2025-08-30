use crate::java::mc::util::math::{BlockPos, Direction, Vec3d};

rosttasse::bind! {
    use net.minecraft.entity.player;

    impl PlayerEntity {
        fn get_block_pos(self) -> BlockPos;
        fn get_facing(self) -> Direction;
        fn get_rotation_vector(self) -> Vec3d;
    }
}
