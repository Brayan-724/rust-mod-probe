use crate::java::mc::entity::Entity;

rosttasse::bind! {
    use net.minecraft.world;

    impl World {
        let is_client: bool;

        fn spawn_entity(self, entity: Entity) -> bool;
    }
}
