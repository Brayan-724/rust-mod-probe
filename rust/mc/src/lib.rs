use crate::item::{Item, ItemGroups};
use rosttasse::jni::JNIEnv;
use rosttasse::prelude::{Function, JClass};

pub mod entity;
pub mod item;
pub mod registry;
pub mod text;
pub mod util;
pub mod world;

rosttasse::bind! {
    use me.apika.apikaprobe;

    impl RustBridge {
        static SERJIO: Item;

        fn register_group_event(event: Event, item: Item);
        fn item_factory(item: JClass) -> Function;
    }
}

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
