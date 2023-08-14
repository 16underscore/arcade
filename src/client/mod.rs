use crate::entity::Player;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_systems(Startup, setup)
		.add_systems(Update, input)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(SceneBundle {
			scene: asset_server.load("map.glb#Scene0"),
			..default()
		})
		.insert(Name::new("Ground"));
	commands
		.spawn((
			Player,
			SceneBundle {
				scene: asset_server.load("player.glb#Scene0"),
				..Default::default()
			},
		))
		.insert(Name::new("Player"));
	commands
		.spawn(SceneBundle {
			scene: asset_server.load("cannon.glb#Scene0"),
			transform: Transform::from_xyz(10.0, 0.0, 5.0),
			..default()
		})
		.insert(Name::new("Cannon"));
	commands
		.spawn(DirectionalLightBundle {
			directional_light: DirectionalLight {
				illuminance: 10000.0,
				shadows_enabled: true,
				..default()
			},
			transform: Transform::from_xyz(-1., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		})
		.insert(Name::new("Sun"));
	commands
		.spawn(Camera3dBundle {
			transform: Transform::from_xyz(-20.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		})
		.insert(Name::new("Camera"));
}

fn input(
	windows: Query<&Window, With<PrimaryWindow>>,
	keyboard: Res<Input<KeyCode>>,
	mut player_transform: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
	mut camera_transform: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
	let window = windows.single();
	if let Some(position) = window.cursor_position() {
		let midwidth = window.width() / 2.0;
		let midheight = window.height() / 2.0;
		let horizontal = (position.x - midwidth) / midwidth;
		let vertical = (position.y - midheight) / midheight;
		for mut transform in &mut player_transform {
			if keyboard.pressed(KeyCode::D) {
				transform.translation.x += vertical;
				transform.translation.z += horizontal;
			}
			if keyboard.pressed(KeyCode::S) {
				transform.translation.x += vertical;
				transform.translation.z -= horizontal;
			}
			if keyboard.pressed(KeyCode::A) {
				transform.translation.x -= vertical;
				transform.translation.z -= horizontal;
			}
			if keyboard.pressed(KeyCode::W) {
				transform.translation.x -= vertical;
				transform.translation.z += horizontal;
			}
			let target = transform.translation + Vec3::new(vertical, 0.0, -horizontal);
			transform.look_at(target, Vec3::Y);
		}
		for mut transform in &mut camera_transform {
			if keyboard.pressed(KeyCode::D) {
				transform.translation.x += vertical;
				transform.translation.z += horizontal;
			}
			if keyboard.pressed(KeyCode::S) {
				transform.translation.x += vertical;
				transform.translation.z -= horizontal;
			}
			if keyboard.pressed(KeyCode::A) {
				transform.translation.x -= vertical;
				transform.translation.z -= horizontal;
			}
			if keyboard.pressed(KeyCode::W) {
				transform.translation.x -= vertical;
				transform.translation.z += horizontal;
			}
		}
	}
}
