use bevy::prelude::*;

mod components;
mod systems;
mod ui;

pub use components::*;
use systems::*;
use ui::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_system(tower_shooting)
            .add_system(tower_button_clicked)
            .add_system(create_ui_on_selection);
    }
}
