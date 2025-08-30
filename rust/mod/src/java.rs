use crate::java::mc::item::{Item, ItemGroups};
use jni::JNIEnv;

pub mod mc;

rosttasse::bind! {
    use net.fabricmc.api.event;

    impl Event {}
}

rosttasse::bind! {
    use net.fabricmc.api.itemgroup.v1;


    impl ItemGroupEvents {
        fn modify_entries_event(registry_key: ItemGroups) -> Event;
    }

    #[rename = "ItemGroupEvents$ModifyEntries"]
    impl ModifyEntries {}
}

impl Event {
    pub fn register<'local>(self, env: &mut JNIEnv<'local>, item: Item) {
        RustBridge::register_group_event(env, self, item)
    }
}
