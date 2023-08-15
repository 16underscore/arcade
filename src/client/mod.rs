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
	gamepads: Res<Gamepads>,
	axes: Res<Axis<GamepadAxis>>,
	windows: Query<&Window, With<PrimaryWindow>>,
	keyboard: Res<Input<KeyCode>>,
	mut transforms: Query<&mut Transform, Or<(With<Player>, With<Camera3d>)>>,
) {
	let window = windows.single();
	if let Some(position) = window.cursor_position() {
		let horizontal = position.x - window.width() / 2.0;
		let vertical = position.y - window.height() / 2.0;
		let direction = Vec2::new(horizontal, -vertical);
		let direction = direction.normalize_or_zero();
		for mut transform in &mut transforms {
			if keyboard.pressed(KeyCode::W) {
				transform.translation.x += direction.x;
				transform.translation.z += direction.y;
			}
			if keyboard.pressed(KeyCode::A) {
				transform.translation.x += direction.x;
				transform.translation.z -= direction.y;
			}
			if keyboard.pressed(KeyCode::S) {
				transform.translation.x -= direction.x;
				transform.translation.z -= direction.y;
			}
			if keyboard.pressed(KeyCode::D) {
				transform.translation.x -= direction.x;
				transform.translation.z += direction.y;
			}
		}
	}
	for gamepad in gamepads.iter() {
		let left_stick_x = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
			.unwrap_or_default();
		let left_stick_y = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
			.unwrap_or_default();
		for mut transform in &mut transforms {
			transform.translation.x += left_stick_y;
			transform.translation.z += left_stick_x;
		}
	}
}
