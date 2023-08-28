use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{entity::*, AppState};

use super::shop::{setup, Shop};

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, input.run_if(in_state(AppState::Game)));
	}
}

fn input(
	mut commands: Commands,
	windows: Query<&Window, With<PrimaryWindow>>,
	keyboard: Res<Input<KeyCode>>,
	mut players: Query<(&Speed, &mut Velocity, &mut Transform), With<Player>>,
	shop: Query<Entity, With<Shop>>,
) {
	let window = windows.single();
	if let Some(position) = window.cursor_position() {
		let x = position.x - window.width() / 2.;
		let z = position.y - window.height() / 2.;
		let (speed, mut velocity, mut transform) = players.single_mut();
		let direction = Vec3::new(x, 0., z).clamp_length(-speed.0, speed.0);
		let pos = transform.translation;
		transform.look_at(pos + direction, Vec3::Y);
		if keyboard.pressed(KeyCode::W) {
			velocity.linvel += Vec3::new(-direction.z, 0., direction.x);
		}
		if keyboard.just_pressed(KeyCode::Space) {
			velocity.linvel += Vec3::Y * 10.;
		}
	}
	if keyboard.just_pressed(KeyCode::E) {
		if shop.is_empty() {
			setup(&mut commands);
		} else {
			for entity in &shop {
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}
