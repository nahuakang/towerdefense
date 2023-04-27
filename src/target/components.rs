use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}
