use bevy::{prelude::*, utils::FloatOrd};

use crate::*;

pub(super) fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;
            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                commands.entity(tower_entity).with_children(|commands| {
                    commands.spawn((
                        SceneBundle {
                            scene: bullet_assets.tomato_scene.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        },
                        Bullet {
                            direction,
                            speed: 2.5,
                        },
                        Lifetime {
                            timer: Timer::from_seconds(0.5, TimerMode::Once),
                        },
                        Name::new("Bullet"),
                    ));
                });
            }
        }
    }
}

fn spawn_tomato_tower(commands: &mut Commands, game_assets: &GameAssets, position: Vec3) -> Entity {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            Name::new("Tomato_Tower"),
            Tower {
                shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                bullet_offset: Vec3::new(0.0, 0.6, 0.0),
            },
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: game_assets.tomato_tower_scene.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
        })
        .id()
}

pub(super) fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    assets: Res<GameAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.selected() {
                commands.entity(entity).despawn_recursive();
                spawn_tomato_tower(&mut commands, &assets, transform.translation);
            }
        }
    }
}
