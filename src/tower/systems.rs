use bevy::{prelude::*, utils::FloatOrd};

use crate::{Bullet, GameAssets, Lifetime, Target, Tower};

pub fn tower_shooting(
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
                            scene: bullet_assets.bullet_scene.clone(),
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
