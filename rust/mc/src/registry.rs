use crate::util::Identifier;

rosttasse::bind! {
    use net.minecraft.registry;

    impl RegistryKey {
        fn of(kind: RegistryKeys, id: Identifier) -> Self;
    }

    enum RegistryKeys: RegistryKey {
        ITEM,
    }
}
