mod game;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use self::{game::GamePlugin, ui::MenuPlugin};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
	#[default]
	Menu,
	Game,
}

pub fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_state::<AppState>()
		.add_plugins((MenuPlugin, GamePlugin))
		.run();
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}
