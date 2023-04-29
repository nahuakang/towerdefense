mod bullet;
mod main_menu;
mod player;
mod target;
mod tower;

pub use bullet::*;
pub use main_menu::*;
pub use player::*;
pub use target::*;
pub use tower::*;

use bevy::{app::AppExit, pbr::NotShadowCaster, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;

pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        // Toggle Egui Inspector State
        .add_state::<InspectorState>()
        // Game State
        .add_state::<GameState>()
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
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new().run_if(in_state(InspectorState::On)))
        .add_plugin(MainMenuPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading.in_base_set(StartupSet::PreStartup))
        // Systems
        .add_system(spawn_basic_scene.in_schedule(OnEnter(GameState::Gameplay)))
        .add_system(camera_controls)
        .add_system(toggle_inspector_egui)
        .add_system(exit_game)
        .run();
}

// === Game Sate ===
#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Gameplay,
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
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    potato_tower_scene: Handle<Scene>,
    potato_scene: Handle<Scene>,
    cabbage_tower_scene: Handle<Scene>,
    cabbage_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
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
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        potato_tower_scene: assets.load("PotatoTower.glb#Scene0"),
        potato_scene: assets.load("Potato.glb#Scene0"),
        cabbage_tower_scene: assets.load("CabbageTower.glb#Scene0"),
        cabbage_scene: assets.load("Cabbage.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}

pub fn camera_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed: f32 = 3.0;
    let rotate_speed: f32 = 0.3;

    if keyboard_input.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard_input.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

pub fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 50.0,
                subdivisions: 0,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    for i in 0..10 {
        for j in 0..2 {
            commands
                .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                    2.0 * i as f32 + j as f32,
                    0.8,
                    5.0 * j as f32,
                )))
                .insert(Name::new("Tower_Base"))
                .insert(meshes.add(shape::Capsule::default().into()))
                .insert(Highlighting {
                    initial: default_collider_color.clone(),
                    hovered: Some(selected_collider_color.clone()),
                    pressed: Some(selected_collider_color.clone()),
                    selected: Some(selected_collider_color.clone()),
                })
                .insert(default_collider_color.clone())
                .insert(NotShadowCaster)
                .insert(PickableBundle::default())
                .with_children(|commands| {
                    commands.spawn(SceneBundle {
                        scene: game_assets.tower_base_scene.clone(),
                        transform: Transform::from_xyz(0.0, -0.8, 0.0),
                        ..Default::default()
                    });
                });
        }
    }

    for i in 1..25 {
        commands
            .spawn(SceneBundle {
                scene: game_assets.target_scene.clone(),
                transform: Transform::from_xyz(-2.0 * i as f32, 0.4, 2.5),
                ..Default::default()
            })
            .insert(Target {
                speed: 0.45,
                ..Default::default()
            })
            .insert(Health { value: 3 })
            .insert(Name::new("Target"));
    }

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
