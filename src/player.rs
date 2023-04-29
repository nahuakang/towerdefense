use crate::*;
use bevy::prelude::*;

use crate::GameState;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    money: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(give_money_on_kill.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(despawn_player.in_schedule(OnExit(GameState::Gameplay)));
    }
}

pub(super) fn spawn_player(mut commands: Commands) {
    commands.spawn((Player { money: 100 }, Name::new("Player")));
}

pub(super) fn despawn_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player_entity in players.iter() {
        commands.entity(player_entity).despawn_recursive();
    }
}

pub(super) fn give_money_on_kill(
    mut player: Query<&mut Player>,
    mut death_events: EventReader<TargetDeathEvent>,
) {
    let mut player = player.single_mut();
    for _event in death_events.iter() {
        player.money += 10;
    }
}
