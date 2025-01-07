mod macros;
pub mod primitives;

use jni::{
    objects::{JObject, JValueOwned},
    JNIEnv,
};

pub use macros::*;
use primitives::Function;

pub trait IntoJValue {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local>;
}

impl IntoJValue for String {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        env.new_string(self).unwrap().into()
    }
}

impl IntoJValue for JObject<'_> {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        env.new_local_ref(self).unwrap().into()
    }
}

pub trait JSignature {
    fn sig() -> String;

    fn sig_type() -> String {
        let sig = Self::sig();
        let mut out = String::with_capacity(sig.len() + 2);

        out.push('L');
        out.push_str(&sig);
        out.push(';');

        out
    }
}

new_class!{RustBridge: "com/apika_probe_1/RustBridge" {
  static registerGroupEvent fn register_group_event(event: Event, item: Item);
}}

new_class! {Event: "net/fabricmc/fabric/api/event/Event" {}}

impl Event<'_> {
    pub fn register<'local>(self, env: &mut JNIEnv<'local>, item: Item) {
        RustBridge::register_group_event(env, self, item)
    }
}

new_class! {Identifier: "net/minecraft/util/Identifier" {
    static fn of(mod_id: String, id: String) -> Identifier<'local>;
}}

new_class! {Item: "net/minecraft/item/Item" {}}

new_enum! {ItemGroups [RegistryKey]: "net/minecraft/item/ItemGroups" {
    REDSTONE
}}

new_class! {ItemGroupEvents: "net/fabricmc/fabric/api/itemgroup/v1/ItemGroupEvents" {
    static modifyEntriesEvent fn modify_entries_event(registry_key: ItemGroups) -> Event<'local>;
}}

new_class! {ItemSettings: "net/minecraft/item/Item$Settings" {}}

new_class! {Items: "net/minecraft/item/Items" {
    static register fn register_raw(key: RegistryKey<'local>, factory: Function<'local>, settings: ItemSettings<'local>) -> Item<'local>;
}}

impl Items<'_> {
    pub fn register<'local>(
        env: &mut JNIEnv<'local>,
        key: RegistryKey<'local>,
        settings: ItemSettings<'local>,
    ) -> Item<'local> {
        let factory = {
            let class = env.find_class("com/apika_probe_1/RustBridge").unwrap();

            env.call_static_method(class, "getItemNew", "()Ljava/util/function/Function;", &[])
                .unwrap()
                .l()
                .unwrap()
        };

        Self::register_raw(env, key, factory.into(), settings)
    }
}

new_class! {ModifyEntries: "net/fabricmc/fabric/api/itemgroup/v1/ItemGroupEvents$ModifyEntries" {}}

new_class! {RegistryKey: "net/minecraft/registry/RegistryKey" {
    static fn of(kind: RegistryKeys, id: Identifier<'local>) -> RegistryKey<'local>;
}}

new_enum! {RegistryKeys [RegistryKey]: "net/minecraft/registry/RegistryKeys" {
    ITEM
}}
