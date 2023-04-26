use std::f32::consts::PI;

use bevy::{app::AppExit, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .register_type::<Tower>()
        .register_type::<Lifetime>()
        .add_state::<InspectorState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new().run_if(in_state(InspectorState::On)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(asset_loading)
        .add_system(toggle_inspector_egui)
        .add_system(exit_game)
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        .run();
}

#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum InspectorState {
    #[default]
    On,
    Off,
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn toggle_inspector_egui(
    keyboard_input: Res<Input<KeyCode>>,
    inspector_state: Res<State<InspectorState>>,
    mut inspector_state_next_state: ResMut<NextState<InspectorState>>,
) {
    if keyboard_input.just_pressed(KeyCode::T) && inspector_state.0 != InspectorState::On {
        inspector_state_next_state.set(InspectorState::On);
    }
    if keyboard_input.just_pressed(KeyCode::T) && inspector_state.0 != InspectorState::Off {
        inspector_state_next_state.set(InspectorState::Off);
    }
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                subdivisions: 0,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Name::new("Ground"),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.6, 0.8, 0.9).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Name::new("Tower"),
    ));
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        Name::new("Light"),
    ));
}

pub fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<&mut Tower>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    if let Ok(mut tower) = towers.get_single_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            // Shoot the bullet
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands.spawn((
                SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..default()
                },
                Lifetime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
                Name::new("Bullet"),
            ));
        }
    }
}

pub fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
