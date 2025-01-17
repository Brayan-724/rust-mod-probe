use probe::{class::Instance, JavaClass};

pub mod passive;
pub mod player;

#[derive(JavaClass)]
#[package(net.minecraft.entity)]
pub struct Entity {
    #[instance]
    pub raw: Instance,
}

#[derive(JavaClass)]
#[package(net.minecraft.entity)]
pub enum EntityType {
    WOLF,
}
