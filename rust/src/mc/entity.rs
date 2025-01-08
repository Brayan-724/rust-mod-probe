use crate::{new_class, new_enum};

pub mod passive;
pub mod player;

new_class! {Entity: "net/minecraft/entity/Entity" {}}

new_enum! {EntityType: "net/minecraft/entity/EntityType" {
    WOLF
}}
