use crate::{
    mc::{util::math::Vec3d, world::World},
    new_class,
};

use super::EntityType;

new_class! {WolfEntity(entity_type: EntityType, world: World): "net/minecraft/entity/passive/WolfEntity" {
    pub setPosition fn set_position(position: Vec3d);
}}
