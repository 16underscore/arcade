use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{entity::*, AppState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, input.run_if(in_state(AppState::Game)));
	}
}

fn input(
	gamepads: Res<Gamepads>,
	axes: Res<Axis<GamepadAxis>>,
	windows: Query<&Window, With<PrimaryWindow>>,
	keyboard: Res<Input<KeyCode>>,
	mut players: Query<(&Speed, &mut Velocity, &mut Transform), With<Player>>,
) {
	let window = windows.single();
	let (speed, mut velocity, mut transform) = players.single_mut();
	if let Some(position) = window.cursor_position() {
		let x = position.x - window.width() / 2.;
		let z = position.y - window.height() / 2.;
		let direction = Vec3::new(x, 0., z).clamp_length(-speed.0, speed.0);
		let pos = transform.translation;
		transform.look_at(pos + direction, Vec3::Y);
		if keyboard.pressed(KeyCode::W) {
			velocity.linvel += Vec3::new(-direction.z, 0., direction.x);
		}
		if keyboard.just_pressed(KeyCode::Space) {
			velocity.linvel += Vec3::Y;
		}
	}
	for gamepad in gamepads.iter() {
		let left_stick_x = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
			.unwrap_or_default();
		let left_stick_y = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
			.unwrap_or_default();
		let _direction = Vec3::new(left_stick_x, 0., left_stick_y).clamp_length(-speed.0, speed.0);
	}
}
