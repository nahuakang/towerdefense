use bevy::prelude::*;

#[derive(Resource)]
pub struct TargetPath {
    pub waypoints: Vec<Vec2>,
}
