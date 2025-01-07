mod java;

use java::{
    Identifier, IntoJValue, Item, ItemGroupEvents, ItemGroups, ItemSettings, Items, JSignature, RegistryKey, RegistryKeys, RustBridge
};
use jni::JNIEnv;

use jni::objects::JClass;

const MOD_ID: &str = "apikaprobe";

fn register_item<'local>(
    env: &mut JNIEnv<'local>,
    id: &str,
    kind: RegistryKeys,
    settings: ItemSettings<'local>,
) -> Item<'local> {
    let id = id.to_owned();
    let mod_id = MOD_ID.to_owned();

    let identifier = Identifier::of(env, mod_id, id);
    let registry_key = RegistryKey::of(env, kind, identifier);

    let value = Items::register(env, registry_key, settings);

    let class = RustBridge::class(env);
    let key = env
        .get_static_field_id(&class, "SUSPICIOUS_SUBSTANCE", Item::sig_type())
        .unwrap();

    {
        let value = (&value).into_jvalue(env);
        env.set_static_field(class, key, value.borrow()).unwrap();
    }

    value
}

#[no_mangle]
pub extern "system" fn Java_me_apika_apikaprobe_RustBridge_main<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    let env = &mut env;

    let settings = ItemSettings::new(env);
    let item = register_item(env, "serjio", RegistryKeys::ITEM, settings);

    ItemGroupEvents::modify_entries_event(env, ItemGroups::REDSTONE).register(env, item);
}
