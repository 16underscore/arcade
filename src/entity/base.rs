use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::OnGameScreen;

#[derive(Bundle)]
pub struct BaseBundle {
	name: Name,
	scene_bundle: SceneBundle,
	game: OnGameScreen,
	rigid_body: RigidBody,
}

impl BaseBundle {
	pub fn new(number: usize, position: Vec3, scene: Handle<Scene>) -> Self {
		Self {
			name: Name::new(format!("Base {}", number)),
			scene_bundle: SceneBundle {
				scene,
				transform: Transform::from_translation(position),
				..default()
			},
			game: OnGameScreen,
			rigid_body: RigidBody::Fixed,
		}
	}
}

#[derive(Component)]
pub struct Base;
