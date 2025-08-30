use rosttasse::{JNIClass, JNIEnv};
use rosttasse_mc::item::{ItemGroups, ItemSettings, Items};
use rosttasse_mc::registry::RegistryKeys;
use rosttasse_mc::ItemGroupEvents;

use crate::items::SerjioItem;

mod items;

// const MOD_ID: &str = "apikaprobe";

// #[rosttasse::main]
// fn main() {
// }

#[no_mangle]
pub extern "system" fn Java_me_apika_apikaprobe_RustBridge_main<'local>(
    mut env: JNIEnv<'local>,
    _class: JNIClass<'local>,
) {
    let env = &mut env;
    let item = Items::register::<SerjioItem>(RegistryKeys::ITEM, ItemSettings::default(env), env);

    ItemGroupEvents::modify_entries_event(ItemGroups::REDSTONE, env).register(item, env);
}

pub use items::Java_me_apika_apikaprobe_SerjioItem_use;

// #[no_mangle]
// pub extern "system" fn Java_me_apika_apikaprobe_SerjioItem_use<'local>(
//     mut env: JNIEnv<'local>,
//     _class: JNIClass<'local>,
//     world: JNIObject<'local>,
//     user: JNIObject<'local>,
//     hand: JNIObject<'local>,
// ) -> JNIObject<'local> {
//     let env = &mut env;
//
//     let world = World::from_raw(world.into());
//     let user = PlayerEntity::from_raw(user.into());
//     let _hand = Hand::from_raw(hand.into());
//
//     if world.is_client.get(env) {
//         return ActionResult::PASS.get_raw(env).l().unwrap();
//     }
//
//     ActionResult::SUCCESS.into_jvalue(env).l().unwrap()
// }
