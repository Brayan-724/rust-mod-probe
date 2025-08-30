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

// use crate::new_class;
//
// use probe::{class::Instance, JavaClass};
//
// #[derive(JavaClass)]
// #[package(net.minecraft.util.math)]
// pub struct BlockPos {
//     #[instance]
//     pub raw: Instance,
// }
//
// #[probe::import]
// impl BlockPos {
//     pub fn offset(self, direction: Direction, distance: i32);
//     pub fn to_center_pos(self) -> Vec3d;
// }
//
// #[derive(JavaClass)]
// #[package(net.minecraft.util.math)]
// pub struct Direction {
//     #[instance]
//     pub raw: Instance,
// }
//
// #[derive(JavaClass)]
// #[package(net.minecraft.util.math)]
// pub struct Vec3d {
//     #[instance]
//     pub raw: Instance,
// }
//
// #[probe::import]
// impl Vec3d {
//     pub extern "multiply" fn scale(self, scale: f64) -> Self;
//     pub extern "add" fn add_vec(self, vec: Self) -> Self;
// }
