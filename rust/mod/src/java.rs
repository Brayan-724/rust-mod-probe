pub mod mc;

use jni::JNIEnv;
use probe::{
    class::Instance,
    primitives::{Function, JClass},
    JavaClass,
};

#[derive(JavaClass)]
#[package(me.apika.apikaprobe)]
struct SerjioItem(#[instance] pub Instance);

#[derive(JavaClass)]
#[package(net.fabricmc.fabric.api.event)]
pub struct Event {
    #[instance]
    pub raw: Instance,
}

impl Event {
    pub fn register<'local>(self, env: &mut JNIEnv<'local>, item: Item) {
        RustBridge::register_group_event(env, self, item)
    }
}

#[derive(JavaClass)]
#[package(net.minecraft.util)]
pub struct Identifier(#[instance] pub Instance);

#[probe::import]
impl Identifier {
    pub fn of(mod_id: String, id: String) -> Identifier;
}

#[derive(JavaClass)]
#[package(net.minecraft.item)]
pub struct Item(#[instance] pub Instance);

// new_enum! {ItemGroups [RegistryKey]: "net/minecraft/item/ItemGroups" {}}
#[derive(JavaClass)]
#[package(net.minecraft.item)]
#[enum_of(RegistryKey)]
pub enum ItemGroups {
    REDSTONE,
}

#[derive(JavaClass)]
#[package(net.fabricmc.fabric.api.itemgroup.v1)]
pub struct ItemGroupEvents(#[instance] pub Instance);

#[probe::import]
impl ItemGroupEvents {
    pub fn modify_entries_event(registry_key: ItemGroups) -> Event;
}

#[derive(JavaClass)]
#[package(net.minecraft.item)]
#[rename("Item$Settings")]
pub struct ItemSettings(#[instance] pub Instance);

#[probe::import]
impl ItemSettings {}

#[derive(JavaClass)]
#[package(net.minecraft.item)]
pub struct Items(#[instance] pub Instance);

#[probe::import]
impl Items {
    pub extern "register" fn register_raw(
        key: RegistryKey,
        factory: Function,
        settings: ItemSettings,
    ) -> Item;
}

impl Items {
    pub fn register<'local>(
        env: &mut JNIEnv<'local>,
        key: RegistryKey,
        settings: ItemSettings,
    ) -> Item {
        let factory = {
            // let class = RustBridge::class(env);
            let item = SerjioItem::class(env);

            RustBridge::item_factory(env, item)

            // env.call_static_method(
            //     class,
            //     "itemFactory",
            //     "(Ljava/lang/Class;)Ljava/util/function/Function;",
            //     &[(&item).into()],
            // )
            // .unwrap()
            // .l()
            // .unwrap()
        };

        Self::register_raw(env, key, factory, settings)
    }
}

#[derive(JavaClass)]
#[package(net.fabricmc.fabric.api.itemgroup.v1)]
#[rename("ItemGroupEvents$ModifyEntries")]
struct ModifyEntries(#[instance] pub Instance);

#[derive(JavaClass)]
#[package(net.minecraft.registry)]
pub struct RegistryKey(#[instance] pub Instance);

#[probe::import]
impl RegistryKey {
    pub fn of(kind: RegistryKeys, id: Identifier) -> RegistryKey;
}

// new_enum! {RegistryKeys [RegistryKey]: "net/minecraft/registry/RegistryKeys" { }}
#[derive(JavaClass)]
#[package(net.minecraft.registry)]
#[enum_of(RegistryKey)]
pub enum RegistryKeys {
    ITEM,
}
