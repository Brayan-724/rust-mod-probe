pub mod math;

rosttasse::bind! {
    use net.minecraft.util;

    enum ActionResult {
        PASS = "Pass",
        SUCCESS = "Success",
    }

    impl Hand {}
}

// use probe::{class::Instance, JavaClass};
//
// pub mod math;
//
// #[derive(JavaClass)]
// #[package(net.minecraft.util)]
// pub enum ActionResult {
//     #[variant = "Pass"]
//     PASS,
//
//     #[variant = "Success"]
//     SUCCESS,
// }
//
// #[derive(JavaClass)]
// #[package(net.minecraft.util)]
// pub struct Hand {
//     #[instance]
//     pub raw: Instance,
// }
