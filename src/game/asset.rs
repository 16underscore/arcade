use bevy::prelude::*;

pub struct GameAssetPlugin;

#[derive(Resource)]
pub struct GameAssets {
	pub scene: Scenes,
	pub mesh: Meshes,
}

pub struct Scenes {
	pub cannon: Handle<Scene>,
	pub map: Handle<Scene>,
	pub player: Handle<Scene>,
}

pub struct Meshes {
	pub map: Handle<Mesh>,
}

impl Plugin for GameAssetPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, load);
	}
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
	let assets = GameAssets {
		scene: Scenes {
			cannon: asset_server.load("entity/cannon.glb#Scene0"),
			map: asset_server.load("map/map.glb#Scene0"),
			player: asset_server.load("entity/player.glb#Scene0"),
		},
		mesh: Meshes {
			map: asset_server.load("map/map.glb#Mesh0/Primitive0"),
		},
	};
	commands.insert_resource(assets);
}
