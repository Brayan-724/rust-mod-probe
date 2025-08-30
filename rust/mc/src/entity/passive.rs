use crate::entity::EntityType;
use crate::text::Text;
use crate::util::math::Vec3d;
use crate::world::World;

rosttasse::bind! {
    use net.minecraft.entity.passive;

    impl WolfEntity {
        #[constructor]
        fn new(entity_type: EntityType, world: World) -> Self;

        fn set_position(self, position: Vec3d);
        fn set_custom_name(self, name: Text);
    }
}
