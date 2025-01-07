mod java;

use java::{
    Identifier, IntoJValue, Item, ItemGroupEvents, ItemGroups, ItemSettings, Items, JSignature, RegistryKey, RegistryKeys
};
use jni::JNIEnv;

use jni::objects::{JClass, JObject, JString};

const MOD_ID: &str = "apika_probe_1";

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

    let class = env.find_class("com/apika_probe_1/RustBridge").unwrap();
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
pub extern "system" fn Java_com_apika_1probe_11_RustBridge_main<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    let env = &mut env;

    let settings = ItemSettings::new(env);
    let item = register_item(env, "suspicious_substance", RegistryKeys::ITEM, settings);

    ItemGroupEvents::modify_entries_event(env, ItemGroups::REDSTONE).register(env, item);
}

#[no_mangle]
pub extern "system" fn Java_com_apika_1probe_11_RustBridge_itemId<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    let output = env
        .new_string("suspicious_substance")
        .expect("Couldn't create java string!");

    output
}

#[no_mangle]
pub extern "system" fn Java_com_apika_1probe_11_RustBridge_itemSettings<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JObject<'local> {
    ItemSettings::new(&mut env).into()
}

#[no_mangle]
pub extern "system" fn Java_com_apika_1probe_11_RustBridge_register<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: JString<'local>,
    settings: JObject<'local>,
) -> JObject<'local> {
    let id = env.get_string(&id).unwrap().into();
    let mod_id = "apika_probe_1".to_owned();

    let identifier = Identifier::of(&mut env, mod_id, id);

    let registry_key = RegistryKey::of(&mut env, RegistryKeys::ITEM, identifier);

    Items::register(&mut env, registry_key, settings.into())
        .into_jvalue(&mut env)
        .l()
        .unwrap()
}
