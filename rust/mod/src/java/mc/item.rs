use crate::java::mc::registry::RegistryKey;
use jni::JNIEnv;
use rosttasse::prelude::Function;

use crate::java::SerjioItem;

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
        let factory = {
            let item = SerjioItem::class(env);

            RustBridge::item_factory(env, item)
        };

        Self::register_raw(key, factory, settings, env)
    }
}
