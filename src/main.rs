mod entity;
mod game;
mod ui;

use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

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
		.add_state::<AppState>()
		.add_plugins((MenuPlugin, GamePlugin))
		.add_plugins(DebugPlugin)
		.run();
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}

struct DebugPlugin;

impl Plugin for DebugPlugin {
	#[allow(unused_variables)]
	fn build(&self, app: &mut App) {
		#[cfg(debug_assertions)]
		app.add_plugins((
			WorldInspectorPlugin::new(),
			RapierDebugRenderPlugin::default(),
		));
	}
}
