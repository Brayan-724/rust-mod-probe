use crate::new_class;

use super::entity::Entity;

new_class! {World: "net/minecraft/world/World" {
    let is_client: bool = isClient;
    pub spawnEntity fn spawn_entity(entity: Entity) -> bool;
}}
