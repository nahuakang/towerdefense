use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_systems(
                (start_button_clicked, quit_button_clicked).in_set(OnUpdate(GameState::MainMenu)),
            );
    }
}

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

pub(super) const MAIN_MENU_STYLE: Style = Style {
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    justify_content: JustifyContent::Center,
    flex_direction: FlexDirection::Column,
    ..Style::DEFAULT
};

pub(super) const BUTTON_STYLE: Style = Style {
    size: Size::new(Val::Percent(65.0), Val::Percent(15.0)),
    align_self: AlignSelf::Center,
    justify_content: JustifyContent::Center,
    margin: UiRect::all(Val::Percent(2.0)),
    ..Style::DEFAULT
};

pub(super) const TEXT_BUNDLE_STYLE: Style = Style {
    align_self: AlignSelf::Center,
    margin: UiRect::all(Val::Percent(3.0)),
    ..Style::DEFAULT
};

pub(super) fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = spawn_button(&mut commands, &asset_server, "Start Game", Color::BLUE);
    commands.entity(start_button).insert(StartButton);

    let quit_button = spawn_button(&mut commands, &asset_server, "Quit", Color::RED);
    commands.entity(quit_button).insert(QuitButton);

    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MenuUIRoot,
        ))
        .with_children(|commands| {
            commands.spawn((TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Tower Defense Tutorial",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 96.0,
                        color: Color::BEIGE,
                    },
                ),
                ..default()
            },));
        })
        .add_child(start_button)
        .add_child(quit_button);
}

pub(super) fn despawn_main_menu(mut commands: Commands, menus: Query<Entity, With<MenuUIRoot>>) {
    for menu_entity in menus.iter() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub(super) fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state_next_state.set(GameState::Gameplay);
        }
    }
}

pub(super) fn quit_button_clicked(
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: BUTTON_STYLE,
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: TEXT_BUNDLE_STYLE,
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        color: Color::BEIGE,
                    },
                ),
                ..default()
            });
        })
        .id()
}
