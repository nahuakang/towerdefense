use bevy::prelude::*;

use crate::TargetDeathEvent;

use super::components::{GamePlayUIRoot, HealthUI, MoneyUI, Player};

pub(super) fn update_player_ui(
    player: Query<&Player>,
    mut money_ui: Query<&mut Text, (With<MoneyUI>, Without<HealthUI>)>,
    mut health_ui: Query<&mut Text, With<HealthUI>>,
) {
    //Won't panic: There must be 1 and only 1 of each of these entities
    let player = player.single();
    let mut money = money_ui.single_mut();
    let mut health = health_ui.single_mut();

    *money = Text::from_section(
        format!("Money: {}", player.money),
        money.sections[0].style.clone(),
    );
    *health = Text::from_section(
        format!("Health: {}", player.health),
        health.sections[0].style.clone(),
    );
}

pub(super) fn spawn_gameplay_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(GamePlayUIRoot)
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexStart,
                        align_self: AlignSelf::FlexStart,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn((
                        TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.2)),
                                ..default()
                            },
                            text: Text::from_section(
                                "Player Money: XX",
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 36.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        },
                        MoneyUI,
                    ));
                    commands.spawn((
                        TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.2)),
                                ..default()
                            },
                            text: Text::from_section(
                                "Player Health: XX",
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 36.0,
                                    color: Color::BLACK,
                                },
                            ),
                            ..default()
                        },
                        HealthUI,
                    ));
                });
        });
}

pub(super) fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            money: 100,
            health: 10,
        },
        Name::new("Player"),
    ));
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
