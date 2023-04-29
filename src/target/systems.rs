use crate::*;
use bevy::{math::Vec3Swizzles, prelude::*};

use super::{events::TargetDeathEvent, resources::TargetPath};

pub(super) fn despawn_targets(mut commands: Commands, targets: Query<Entity, With<Target>>) {
    for target_entity in targets.iter() {
        commands.entity(target_entity).despawn_recursive();
    }
}

pub(super) fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform)>,
    path: Res<TargetPath>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        // This step will get us closer to the goal
        if delta_target.length() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();
            //Copy for ownership reasons
            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            // At current step
            target.path_index += 1;
        }
    }
}

pub(super) fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut target_death_event_writer: EventWriter<TargetDeathEvent>,
) {
    for (entity, health) in &targets {
        if health.value <= 0 {
            target_death_event_writer.send(TargetDeathEvent);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub(super) fn hurt_player(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<TargetPath>,
    mut player: Query<&mut Player>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            commands.entity(entity).despawn_recursive();

            audio.play(asset_server.load("damage.wav"));

            let mut player = player.single_mut();
            if player.health > 0 {
                player.health -= 1;
            }

            if player.health == 0 {
                info!("GAME OVER");
            }
        }
    }
}
