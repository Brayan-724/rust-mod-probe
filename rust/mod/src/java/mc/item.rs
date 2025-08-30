use crate::java::mc::registry::RegistryKey;
use crate::{RustBridge, SerjioItem};
use jni::JNIEnv;
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
    pub fn register<'local>(
        key: RegistryKey,
        settings: ItemSettings,
        env: &mut JNIEnv<'local>,
    ) -> Item {
        let factory = RustBridge::item_factory(SerjioItem::class(env), env);

        Self::register_raw(key, factory, settings, env)
    }
}
