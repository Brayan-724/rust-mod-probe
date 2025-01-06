use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

pub const JAVA_STRING: &str = "java/lang/String";

pub struct Identifier;

impl Identifier {
    pub const MINECRAFT_IDENTIFIER: &str = "net/minecraft/util/Identifier";
    pub const MINECRAFT_IDENTIFIER_OF: &str =
        "(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/util/Identifier;";

    pub fn of<'local>(
        env: &mut JNIEnv<'local>,
        mod_id: JString<'local>,
        id: JString<'local>,
    ) -> JObject<'local> {
        let identifier_class = env
            .find_class(Identifier::MINECRAFT_IDENTIFIER)
            .expect("Cannot get Identifier class");

        env.call_static_method(
            identifier_class,
            "of",
            Identifier::MINECRAFT_IDENTIFIER_OF,
            &[(&mod_id).into(), (&id).into()],
        )
        .expect("Cannot generate Identifier")
        .l()
        .expect("Cannot convert to JObject")
    }
}

pub struct Items;

impl Items {
    pub const MINECRAFT_ITEM: &str = "net/minecraft/item/Item";
    pub const MINECRAFT_ITEMS: &str = "net/minecraft/item/Items";
    pub const MINECRAFT_ITEMS_REGISTER: &str =
        "(Lnet/minecraft/registry/RegistryKey;Ljava/util/function/Function;Lnet/minecraft/item/Item$Settings;)Lnet/minecraft/item/Item;";

    pub fn register<'local>(
        env: &mut JNIEnv<'local>,
        key: JObject<'local>,
        settings: JObject<'local>,
    ) -> JObject<'local> {
        let items_class = env
            .find_class(Items::MINECRAFT_ITEMS)
            .expect("Cannot get Items class");

        let item_class = env
            .find_class(Items::MINECRAFT_ITEM)
            .expect("Cannot get Item class");

        let factory = {
            let class = env.find_class("com/apika_probe_1/RustBridge").unwrap();

            env.call_static_method(class, "getItemNew", "()Ljava/util/function/Function;", &[])
                .unwrap()
                .l()
                .unwrap()
        };

        // let factory = env
        //     // .get_static_field(&items_class, "new", "Ljava/util/function/Function;")
        //     .get_static_field(&items_class, "new", "Lnet/minecraft/item/Item$Item;")
        //     // .get_static_method_id(&items_class, "new", "(Lnet/minecraft/item/Item$Settings;)Lnet/minecraft/item/Item;")
        //     .unwrap();

        // let factory = unsafe {
        //     let jni_interface = (*env.get_native_interface()).as_ref().unwrap();
        //     // fn(_: *mut *const JNINativeInterface_, _: *mut _jobject, _: *mut _jmethodID, _: u8)
        //     let to_reflected_method = jni_interface.ToReflectedMethod.unwrap();
        //     let factory = to_reflected_method(
        //         env.get_native_interface(),
        //         **items_class,
        //         factory.into_raw(),
        //         0,
        //     );
        //     JObject::from_raw(factory)
        // };

        env.call_static_method(
            items_class,
            "register",
            Items::MINECRAFT_ITEMS_REGISTER,
            &[(&key).into(), (&factory).into(), (&settings).into()],
        )
        .expect("Cannot generate Item")
        .l()
        .expect("Cannot convert to JObject")
    }
}

pub struct RegistryKey;

impl RegistryKey {
    pub const MINECRAFT_REGISTRYKEY: &str = "net/minecraft/registry/RegistryKey";
    pub const MINECRAFT_REGISTRYKEYS: &str = "net/minecraft/registry/RegistryKeys";

    pub const MINECRAFT_REGISTRYKEY_OF: &str = "(Lnet/minecraft/registry/RegistryKey;Lnet/minecraft/util/Identifier;)Lnet/minecraft/registry/RegistryKey;";

    pub fn of<'local>(
        env: &mut JNIEnv<'local>,
        kind: JObject<'local>,
        id: JObject<'local>,
    ) -> JObject<'local> {
        let registry_class = env
            .find_class(RegistryKey::MINECRAFT_REGISTRYKEY)
            .expect("Cannot get RegistryKey class");

        env.call_static_method(
            registry_class,
            "of",
            RegistryKey::MINECRAFT_REGISTRYKEY_OF,
            &[(&kind).into(), (&id).into()],
        )
        .expect("Cannot generate RegistryKey")
        .l()
        .expect("Cannot convert to JObject")
    }
}
