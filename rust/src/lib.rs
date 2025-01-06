mod java;

use java::{Identifier, RegistryKey, Items};
use jni::JNIEnv;

use jni::objects::{JClass, JObject, JString};

#[no_mangle]
pub extern "system" fn Java_com_apika_1probe_11_RustBridge_itemId<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    let output = env
        .new_string(format!("suspicious_substance"))
        .expect("Couldn't create java string!");

    output
}

#[no_mangle]
pub extern "system" fn Java_com_apika_1probe_11_RustBridge_register<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: JString<'local>,
    settings: JObject<'local>
) -> JObject<'local> {
    let mod_id = env.new_string("apika_probe_1").unwrap();

    let identifier = Identifier::of(&mut env, mod_id, id);

    let registries_class = env
        .find_class(RegistryKey::MINECRAFT_REGISTRYKEYS)
        .expect("Cannot get RegistryKeys class");

    let kind = env
        .get_static_field(registries_class, "ITEM", format!("L{};", RegistryKey::MINECRAFT_REGISTRYKEY))
        .unwrap()
        .l()
        .unwrap();

    let registry_key = RegistryKey::of(&mut env, kind, identifier);

    Items::register(&mut env, registry_key, settings)
}
