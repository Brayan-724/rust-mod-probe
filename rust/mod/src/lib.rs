// mod java;
// mod mc;

// use java::mc::entity::passive::WolfEntity;
// use java::mc::entity::player::PlayerEntity;
// use java::mc::entity::EntityType;
// use java::mc::text::Text;
// use java::mc::util::{ActionResult, Hand};
// use java::mc::world::World;
// use java::{
//     Identifier, Item, ItemGroupEvents, ItemGroups, ItemSettings, Items, RegistryKey, RegistryKeys,
//     RustBridge,
// };
// use jni::JNIEnv;

// use jni::objects::{JClass, JObject};
// use mc::entity::passive::WolfEntity;
// use mc::entity::player::PlayerEntity;
// use mc::entity::EntityType;
// use mc::util::{ActionResult, Hand};
// use mc::world::World;

// use rosttasse::conversion::IntoJValue;
// use rosttasse::JSignature;

const MOD_ID: &str = "apikaprobe";

// fn register_item<'local>(
//     env: &mut JNIEnv<'local>,
//     id: &str,
//     kind: RegistryKeys,
//     settings: ItemSettings,
// ) -> Item {
//     let id = id.to_owned();
//     let mod_id = MOD_ID.to_owned();
//
//     let identifier = Identifier::of(env, mod_id, id);
//     let registry_key = RegistryKey::of(env, kind, identifier);
//
//     let value = Items::register(env, registry_key, settings);
//
//     let class = RustBridge::class(env).into_class();
//     let key = env
//         .get_static_field_id(&class, "SERJIO", Item::sig())
//         .unwrap();
//
//     {
//         let value = value.clone().into_jvalue(env);
//         env.set_static_field(class, key, value.borrow()).unwrap();
//     }
//
//     value
// }

#[rosttasse::bridge("me.apika.apikaprobe", entry, template_file = "./bridge.j")]
mod RustBridge {
    struct Class;

    impl Class {
        #[export]
        extern fn main() { 
            println!("Hello From Rust!")
            panic!()
        }
    }
}

// #[probe::exports]
// pub mod bindings {
//     use jni::JNIEnv;
//
//     use crate::{
//         java::{ItemGroupEvents, ItemGroups, ItemSettings, RegistryKeys},
//         register_item,
//     };
//
//     #[probe::class("me.apika.apikaprobe")]
//     #[entry]
//     #[template_file("./bridge.j")]
//     pub struct RustBridge;
//
//     impl RustBridge {
//         pub extern fn main<'local>(env: &mut JNIEnv<'local>) {
//             let settings = ItemSettings::new(env);
//             let item = register_item("serjio", RegistryKeys::ITEM, settings, env);
//
//             ItemGroupEvents::modify_entries_event(ItemGroups::REDSTONE, env).register(env, item);
//         }
//     }
// }

// #[derive(JavaClass)]
// #[package(me.apika.apikaprobe)]
// #[probe::class("me.apika.apikaprobe")]
// pub struct RustBridge {
//     #[static]
//     pub SERJIO: Item,
// }

// #[probe::import]
// impl RustBridge {
//     // Automatic casing
//     pub fn register_group_event(event: Event, item: Item);
//
//     // Manual casing
//     pub extern "itemFactory" fn item_factory(item: JClass) -> Function;
// }

// #[no_mangle]
// pub extern "system" fn Java_me_apika_apikaprobe_RustBridge_main<'local>(
//     mut env: JNIEnv<'local>,
//     _class: JClass<'local>,
// ) {
//     let env = &mut env;
//
//     let settings = ItemSettings::new(env);
//     let item = register_item(env, "serjio", RegistryKeys::ITEM, settings);
//
//     ItemGroupEvents::modify_entries_event(env, ItemGroups::REDSTONE).register(env, item);
// }
//
// #[no_mangle]
// pub extern "system" fn Java_me_apika_apikaprobe_SerjioItem_use<'local>(
//     mut env: JNIEnv<'local>,
//     _class: JClass<'local>,
//     world: JObject<'local>,
//     user: JObject<'local>,
//     hand: JObject<'local>,
// ) -> JObject<'local> {
//     let env = &mut env;
//
//     let world = World::from_instance(world.into());
//     let user = PlayerEntity::from_instance(user.into());
//     let _hand = Hand::from_instance(hand.into());
//
//     // Ensure we don't spawn the lightning only on the client.
//     // This is to prevent desync.
//     if world.is_client.get(env) {
//         return ActionResult::PASS.get_raw(env).l().unwrap();
//     }
//
//     let player_pos = user.get_block_pos(env).to_center_pos(env);
//     let pos = user
//         .get_rotation_vector(env)
//         .scale(env, 5.0)
//         .add_vec(env, player_pos);
//
//     let wolf = WolfEntity::new(env, EntityType::WOLF, world);
//
//     wolf.set_position(env, pos);
//     let name = Text::of(env, "Serjio".to_string());
//     wolf.set_custom_name(env, name);
//     world.spawn_entity(env, wolf.cast());
//
//     ActionResult::SUCCESS.into_jvalue(env).l().unwrap()
// }
