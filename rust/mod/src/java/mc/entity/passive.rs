use crate::java::mc::entity::EntityType;
use crate::java::mc::text::Text;
use crate::java::mc::util::math::Vec3d;
use crate::java::mc::world::World;

rosttasse::bind! {
    use net.minecraft.entity.passive;

    impl WolfEntity {
        #[constructor]
        fn new(entity_type: EntityType, world: World) -> Self;

        fn set_position(self, position: Vec3d);
        fn set_custom_name(self, name: Text);
    }
}
