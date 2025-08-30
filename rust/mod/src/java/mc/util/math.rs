rosttasse::bind! {
    use net.minecraft.util.math;

    impl BlockPos {
        fn offset(self, direction: Direction, distance: i32);
        fn to_center_pos(self) -> Vec3d;
    }

    impl Direction {}

    impl Vec3d {
        #[rename = "multiply"]
        fn scale(self, scale: f64) -> Self;

        #[rename = "add"]
        fn add_vec(self, lhs: Self) -> Self;
    }
}
