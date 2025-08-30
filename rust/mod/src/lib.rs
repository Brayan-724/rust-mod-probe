mod java;

use java::{mc::item::Item, Event};
use jni::objects::{JClass as JNIClass, JObject as JNIObject};
use jni::JNIEnv;
use rosttasse::prelude::IntoJValue as _;
use rosttasse::primitives::{Function, JClass};
use rosttasse::JavaClass as _;

use crate::java::mc::entity::passive::WolfEntity;
use crate::java::mc::entity::player::PlayerEntity;
use crate::java::mc::entity::EntityType;
use crate::java::mc::item::{ItemGroups, ItemSettings, Items};
use crate::java::mc::registry::RegistryKeys;
use crate::java::mc::text::Text;
use crate::java::mc::util::{ActionResult, Hand};
use crate::java::mc::world::World;
use crate::java::ItemGroupEvents;

const MOD_ID: &str = "apikaprobe";

rosttasse::bind! {
    use me.apika.apikaprobe;

    impl SerjioItem {}

    impl RustBridge {
        static SERJIO: Item;

        fn register_group_event(event: Event, item: Item);
        fn item_factory(item: JClass) -> Function;
    }
}

// #[rosttasse::bridge("me.apika.apikaprobe", entry, template_file = "./bridge.j")]
// mod RustBridge {
//     struct Class;
//
//     impl Class {
//         #[export]
//         extern "C" fn main() {
//             println!("Hello From Rust!");
//             panic!()
//         }
//     }
// }

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

#[no_mangle]
pub extern "system" fn Java_me_apika_apikaprobe_RustBridge_main<'local>(
    mut env: JNIEnv<'local>,
    _class: JNIClass<'local>,
) {
    let env = &mut env;

    let item =
        Items::register::<SerjioItem>(RegistryKeys::ITEM.get(env), ItemSettings::default(env), env);

    ItemGroupEvents::modify_entries_event(ItemGroups::REDSTONE, env).register(item, env);
}

#[no_mangle]
pub extern "system" fn Java_me_apika_apikaprobe_SerjioItem_use<'local>(
    mut env: JNIEnv<'local>,
    _class: JNIClass<'local>,
    world: JNIObject<'local>,
    user: JNIObject<'local>,
    hand: JNIObject<'local>,
) -> JNIObject<'local> {
    let env = &mut env;

    let world = World::from_raw(world.into());
    let user = PlayerEntity::from_raw(user.into());
    let _hand = Hand::from_raw(hand.into());

    // Ensure we don't spawn the lightning only on the client.
    // This is to prevent desync.
    if world.is_client.get(env) {
        return ActionResult::PASS.get_raw(env).l().unwrap();
    }

    let player_pos = user.get_block_pos(env).to_center_pos(env);
    let pos = user
        .get_rotation_vector(env)
        .scale(5.0, env)
        .add_vec(player_pos, env);

    let wolf = WolfEntity::new(EntityType::WOLF, world, env);

    wolf.set_position(pos, env);
    let name = Text::of("Serjio".to_string(), env);
    wolf.set_custom_name(name, env);
    world.spawn_entity(wolf.cast(), env);

    ActionResult::SUCCESS.into_jvalue(env).l().unwrap()
}
