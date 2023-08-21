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
	speeds: Query<&Speed, With<Player>>,
	mut velocities: Query<&mut Velocity, With<Player>>,
) {
	let window = windows.single();
	let Speed(speed) = speeds.single();
	if let Some(position) = window.cursor_position() {
		let halfwidth = window.width() / 2.0;
		let halfheight = window.height() / 2.0;
		let horizontal = (position.x - halfwidth) / halfwidth;
		let vertical = (position.y - halfheight) / halfheight;
		let (x, z) = calc(vertical, horizontal, 4.0);
		let mut velocity = velocities.single_mut();
		if keyboard.pressed(KeyCode::W) {
			velocity.linvel += Vec3::new(-x * speed * 5., 0.0, z * speed * 5.);
		}
		if keyboard.just_pressed(KeyCode::Space) {
			velocity.linvel += Vec3::new(0., 15., 0.);
		}
	}
	for gamepad in gamepads.iter() {
		let left_stick_x = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
			.unwrap_or_default();
		let left_stick_y = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
			.unwrap_or_default();
		let (x, z) = calc(left_stick_y, left_stick_x, 1.25);
		let mut velocity = velocities.single_mut();
		velocity.linvel = Vec3::new(x * speed * 50., 0.0, z * speed * 50.);
	}
}

fn calc(v: f32, h: f32, cursor_speed_distance: f32) -> (f32, f32) {
	let a = v * cursor_speed_distance;
	let b = h * cursor_speed_distance;
	let m = if a < 0.0 { -1.0 } else { 1.0 };
	let n = if b < 0.0 { -1.0 } else { 1.0 };
	let x = f32::max(m * a, n * b).min(1.0) / (m * a + n * b);
	(x * a, x * b)
}
