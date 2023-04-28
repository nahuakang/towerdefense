use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::*;

use super::systems::spawn_tower;

pub(super) fn tower_button_clicked(
    interactions: Query<(&Interaction, &TowerType), Changed<Interaction>>,
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    assets: Res<GameAssets>,
) {
    for (interaction, tower_type) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selection {
                if selection.selected() {
                    commands.entity(entity).despawn_recursive();
                }

                spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
            }
        }
    }
}

pub(super) fn create_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let button_icons = [
        asset_server.load("tomato_tower.png"),
        asset_server.load("potato_tower.png"),
        asset_server.load("cabbage_tower.png"),
    ];

    let towers = [TowerType::Tomato, TowerType::Potato, TowerType::Cabbage];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            TowerUIRoot,
        ))
        .with_children(|commands| {
            for i in 0..button_icons.len() {
                commands.spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexEnd,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        image: button_icons[i].clone().into(),
                        ..default()
                    },
                    towers[i],
                ));
            }
        });
}

pub(super) fn create_ui_on_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Query<&Selection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.selected());

    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        Err(QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(&mut commands, &asset_server);
            }
        }
        _ => unreachable!("Too many UI Tower Roots!"),
    }
}
