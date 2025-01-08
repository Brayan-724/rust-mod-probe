mod macros;
pub mod primitives;

use std::marker::PhantomData;

use jni::{
    objects::{JObject, JValueOwned},
    sys::jobject,
    JNIEnv,
};

pub use macros::*;
use primitives::Function;

pub trait IntoJValue {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local>;
}

impl IntoJValue for i32 {
    fn into_jvalue<'local>(self, _: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        JValueOwned::Int(self)
    }
}

impl IntoJValue for f64 {
    fn into_jvalue<'local>(self, _: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        JValueOwned::Double(self)
    }
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

pub trait FromJValue {
    fn from_jvalue<'local>(value: JValueOwned<'local>) -> Self;
}

impl FromJValue for bool {
    fn from_jvalue<'local>(value: JValueOwned<'local>) -> Self {
        value.z().unwrap()
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

pub struct Field<T>(
    pub(crate) jobject,
    pub(crate) &'static str,
    pub(crate) PhantomData<T>,
);

impl<T: JSignature + FromJValue> Field<T> {
    pub fn get<'local>(&self, env: &mut JNIEnv<'local>) -> T {
        let obj = unsafe { JObject::<'_>::from_raw(self.0) };
        let field = env.get_field(obj, self.1, T::sig_type()).unwrap();
        T::from_jvalue(field)
    }
}

new_class! {RustBridge: "me/apika/apikaprobe/RustBridge" {
  static registerGroupEvent fn register_group_event(event: Event, item: Item);
}}

new_class! {SerjioItem: "me/apika/apikaprobe/SerjioItem" {}}

new_class! {Event: "net/fabricmc/fabric/api/event/Event" {}}

impl Event {
    pub fn register<'local>(self, env: &mut JNIEnv<'local>, item: Item) {
        RustBridge::register_group_event(env, self, item)
    }
}

new_class! {Identifier: "net/minecraft/util/Identifier" {
    static fn of(mod_id: String, id: String) -> Identifier;
}}

new_class! {Item: "net/minecraft/item/Item" {}}

new_enum! {ItemGroups [RegistryKey]: "net/minecraft/item/ItemGroups" {
    REDSTONE
}}

new_class! {ItemGroupEvents: "net/fabricmc/fabric/api/itemgroup/v1/ItemGroupEvents" {
    static modifyEntriesEvent fn modify_entries_event(registry_key: ItemGroups) -> Event;
}}

new_class! {ItemSettings: "net/minecraft/item/Item$Settings" {}}

new_class! {Items: "net/minecraft/item/Items" {
    static register fn register_raw(key: RegistryKey, factory: Function, settings: ItemSettings) -> Item;
}}

impl Items {
    pub fn register<'local>(
        env: &mut JNIEnv<'local>,
        key: RegistryKey,
        settings: ItemSettings,
    ) -> Item {
        let factory = {
            let class = RustBridge::class(env);
            let item = SerjioItem::class(env);

            env.call_static_method(
                class,
                "itemFactory",
                "(Ljava/lang/Class;)Ljava/util/function/Function;",
                &[(&item).into()],
            )
            .unwrap()
            .l()
            .unwrap()
        };

        Self::register_raw(env, key, factory.into(), settings)
    }
}

new_class! {ModifyEntries: "net/fabricmc/fabric/api/itemgroup/v1/ItemGroupEvents$ModifyEntries" {}}

new_class! {RegistryKey: "net/minecraft/registry/RegistryKey" {
    static fn of(kind: RegistryKeys, id: Identifier) -> RegistryKey;
}}

new_enum! {RegistryKeys [RegistryKey]: "net/minecraft/registry/RegistryKeys" {
    ITEM
}}
