use bevy::prelude::*;

use crate::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Component)]
pub struct TowerUIRoot;

// Migration guide from`Inspectable` to `Reflect`: https://shorturl.at/gjq09
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum TowerType {
    #[default]
    Tomato,
    Potato,
    Cabbage,
}

impl TowerType {
    pub(super) fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::Tomato => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Potato => (
                assets.potato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Cabbage => (
                assets.cabbage_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
        }
    }

    pub(super) fn get_bullet(
        &self,
        direction: Vec3,
        assets: &GameAssets,
    ) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::Tomato => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
            TowerType::Potato => (
                assets.potato_scene.clone(),
                Bullet {
                    direction,
                    speed: 6.5,
                },
            ),
            TowerType::Cabbage => (
                assets.cabbage_scene.clone(),
                Bullet {
                    direction,
                    speed: 2.5,
                },
            ),
        }
    }
}
