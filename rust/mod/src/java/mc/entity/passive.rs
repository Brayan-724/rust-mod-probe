use probe::{class::Instance, JavaClass};

use crate::java::mc::{text::Text, util::math::Vec3d, world::World};

use super::EntityType;

#[derive(JavaClass)]
#[package(net.minecraft.entity.passive)]
pub struct WolfEntity {
    #[instance]
    pub raw: Instance,
}

#[probe::import]
impl WolfEntity {
    #[constructor]
    pub fn new(entity_type: EntityType, world: World);

    pub fn set_position(self, position: Vec3d);
    pub fn set_custom_name(self, name: Text);
}
