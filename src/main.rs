mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

use bevy::{app::AppExit, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        // Toggle Egui Inspector State
        .add_state::<InspectorState>()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // Plugins
        .add_plugin(WorldInspectorPlugin::new().run_if(in_state(InspectorState::On)))
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(asset_loading)
        // Systems
        .add_system(toggle_inspector_egui)
        .add_system(exit_game)
        .run();
}

// === Game-level states ===

#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum InspectorState {
    #[default]
    On,
    Off,
}

// === Game-level resources ===

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

// === Game-level systems ===

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
    // Adding ground plane
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
    // Adding tower
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.6, 0.8, 0.9).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        },
        Name::new("Tower"),
    ));
    // Adding targets
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        },
        Target { speed: 0.3 },
        Health { value: 3 },
        Name::new("Target"),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-4.0, 0.2, 1.5),
            ..default()
        },
        Target { speed: 0.3 },
        Health { value: 3 },
        Name::new("Target"),
    ));
    // Adding lighting
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
