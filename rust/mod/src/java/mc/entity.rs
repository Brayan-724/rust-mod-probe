pub mod passive;
pub mod player;

rosttasse::bind! {
    use net.minecraft.entity;

    impl Entity {
    }

    enum EntityType {
        WOLF
    }
}
// use probe::{class::Instance, JavaClass};
//
// pub mod passive;
// pub mod player;
//
// #[derive(JavaClass)]
// #[package(net.minecraft.entity)]
// pub struct Entity {
//     #[instance]
//     pub raw: Instance,
// }
//
// #[derive(JavaClass)]
// #[package(net.minecraft.entity)]
// pub enum EntityType {
//     WOLF,
// }
