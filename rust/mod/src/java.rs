use crate::java::mc::item::{Item, ItemGroups};
use crate::RustBridge;
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
    pub fn register<'local>(self, item: Item, env: &mut JNIEnv<'local>) {
        RustBridge::register_group_event(self, item, env)
    }
}
