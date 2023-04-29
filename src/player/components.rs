use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub money: u32,
    pub health: u32,
}

#[derive(Component)]
pub struct GamePlayUIRoot;

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct MoneyUI;
