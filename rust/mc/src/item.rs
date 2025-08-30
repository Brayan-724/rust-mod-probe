use crate::registry::RegistryKey;
use crate::RustBridge;
use rosttasse::jni::JNIEnv;
use rosttasse::prelude::Function;
use rosttasse::JSignature;

rosttasse::bind! {
    use net.minecraft.item;

    impl Item {}

    enum ItemGroups: RegistryKey {
        REDSTONE
    }

    #[rename = "Item$Settings"]
    impl ItemSettings {}

    impl Items {
        #[rename = "register"]
        fn register_raw(
            key: RegistryKey,
            factory: Function,
            settings: ItemSettings,
        ) -> Item;
    }
}

impl Items {
    pub fn register<'local, ITEM: JSignature>(
        key: RegistryKey,
        settings: ItemSettings,
        env: &mut JNIEnv<'local>,
    ) -> Item {
        let factory = RustBridge::item_factory(ITEM::class(env), env);

        Self::register_raw(key, factory, settings, env)
    }
}
