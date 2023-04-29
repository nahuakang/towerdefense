use crate::*;
use bevy::prelude::*;

use super::events::TargetDeathEvent;

pub(super) fn despawn_targets(mut commands: Commands, targets: Query<Entity, With<Target>>) {
    for target_entity in targets.iter() {
        commands.entity(target_entity).despawn_recursive();
    }
}

pub(super) fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
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
