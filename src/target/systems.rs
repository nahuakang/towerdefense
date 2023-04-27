use crate::*;
use bevy::prelude::*;

pub(super) fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

pub(super) fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (entity, health) in &targets {
        if health.value <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
