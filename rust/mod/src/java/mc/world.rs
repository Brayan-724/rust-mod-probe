// TODO:
// #![package(net.minecraft.world)]

use probe::{
    class::{Field, Instance},
    JavaClass,
};

use super::entity::Entity;

#[derive(Clone, Copy, JavaClass)]
#[package(net.minecraft.world)]
pub struct World {
    #[instance]
    pub raw: Instance,

    pub is_client: Field<bool>,
}

#[probe::import]
impl World {
    pub fn spawn_entity(self, entity: Entity) -> bool;
}

// new_class! {World: "net/minecraft/world/World" {
//     let is_client: bool = isClient;
//     pub spawnEntity fn spawn_entity(entity: Entity) -> bool;
// }}
