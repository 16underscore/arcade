use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::OnGameScreen;

#[derive(Bundle)]
pub struct MapBundle {
	name: Name,
	scene_bundle: SceneBundle,
	game: OnGameScreen,
	rigid_body: RigidBody,
	collider: Collider,
}

impl MapBundle {
	pub fn new(scene: Handle<Scene>, mesh: &Mesh) -> Self {
		Self {
			name: Name::new("Map"),
			scene_bundle: SceneBundle { scene, ..default() },
			game: OnGameScreen,
			rigid_body: RigidBody::Fixed,
			collider: Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull)
				.expect("meshes not loaded"),
		}
	}
}
