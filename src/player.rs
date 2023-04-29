mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(
                (spawn_player, spawn_gameplay_ui).in_schedule(OnEnter(GameState::Gameplay)),
            )
            .add_systems(
                (give_money_on_kill, update_player_ui).in_set(OnUpdate(GameState::Gameplay)),
            )
            .add_system(despawn_player.in_schedule(OnExit(GameState::Gameplay)));
    }
}
