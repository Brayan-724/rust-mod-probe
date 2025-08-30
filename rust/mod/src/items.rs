use rosttasse_mc::entity::passive::WolfEntity;
use rosttasse_mc::entity::player::PlayerEntity;
use rosttasse_mc::entity::EntityType;
use rosttasse_mc::text::Text;
use rosttasse_mc::util::{ActionResult, Hand};
use rosttasse_mc::world::World;

rosttasse::bind! {
    use me.apika.apikaprobe;

    impl SerjioItem {}
}

// rosttasse::register_item! {SerjioItem =>
//     key = RegistryKeys::ITEM,
//     settings = ItemSettings::default,
//     groups = [ItemGroups::REDSTONE]
// }

#[rosttasse::export(me.apika.apikaprobe)]
impl SerjioItem {
    pub fn r#use(self, world: World, user: PlayerEntity, _hand: Hand) -> ActionResult {
        // Ensure we don't spawn the wolf only on the client.
        // This is to prevent desync.
        if world.is_client.get(env) {
            return ActionResult::PASS;
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

        ActionResult::SUCCESS
    }
}
