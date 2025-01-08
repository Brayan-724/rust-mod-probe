mod java;
mod mc;

use java::{
    Identifier, IntoJValue, Item, ItemGroupEvents, ItemGroups, ItemSettings, Items, JSignature,
    RegistryKey, RegistryKeys, RustBridge,
};
use jni::JNIEnv;

use jni::objects::{JClass, JObject};
use mc::entity::passive::WolfEntity;
use mc::entity::player::PlayerEntity;
use mc::entity::EntityType;
use mc::util::{ActionResult, Hand};
use mc::world::World;

const MOD_ID: &str = "apikaprobe";

fn register_item<'local>(
    env: &mut JNIEnv<'local>,
    id: &str,
    kind: RegistryKeys,
    settings: ItemSettings,
) -> Item {
    let id = id.to_owned();
    let mod_id = MOD_ID.to_owned();

    let identifier = Identifier::of(env, mod_id, id);
    let registry_key = RegistryKey::of(env, kind, identifier);

    let value = Items::register(env, registry_key, settings);

    let class = RustBridge::class(env);
    let key = env
        .get_static_field_id(&class, "SERJIO", Item::sig_type())
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

#[no_mangle]
pub extern "system" fn Java_me_apika_apikaprobe_SerjioItem_use<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    world: JObject<'local>,
    user: JObject<'local>,
    hand: JObject<'local>,
) -> JObject<'local> {
    let env = &mut env;

    let world: World = world.into();
    let user: PlayerEntity = user.into();
    let _hand: Hand = hand.into();

    // Ensure we don't spawn the lightning only on the client.
    // This is to prevent desync.
    if world.is_client().get(env) {
        return ActionResult::PASS.into_jvalue(env).l().unwrap();
    }

    let player_pos = user.get_block_pos(env).to_center_pos(env);
    let pos = user
        .get_rotation_vector(env)
        .multiply(env, 5.0)
        .add(env, player_pos);

    let wolf = WolfEntity::new(env, EntityType::WOLF, world);

    wolf.set_position(env, pos);
    // wolf.setCustomName(Text.of("Serjio"));
    world.spawn_entity(env, wolf.cast_by_object());

    ActionResult::SUCCESS.into_jvalue(env).l().unwrap()
}
