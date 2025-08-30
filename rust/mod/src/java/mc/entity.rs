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
