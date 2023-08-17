use bevy::prelude::*;

use crate::{entity::Player, AppState};

pub struct Camera3dPlugin;

impl Plugin for Camera3dPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(super::AppState::Game), setup_camera3d)
			.add_systems(Update, move_camera3d.run_if(in_state(AppState::Game)));
	}
}

fn setup_camera3d(mut commands: Commands) {
	commands
		.spawn((
			Camera3dBundle {
				transform: Transform::from_xyz(-20.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
				..default()
			},
			super::OnGameScreen,
		))
		.insert(Name::new("Camera"));
}

fn move_camera3d(
	mut camera_transforms: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
	player_transforms: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
	if let Ok(mut camera_transform) = camera_transforms.get_single_mut() {
		if let Ok(player_transform) = player_transforms.get_single() {
			camera_transform.look_at(player_transform.translation, Vec3::Y);
		}
	}
}
