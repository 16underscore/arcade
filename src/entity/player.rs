use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::OnGameScreen;

use super::{Health, Speed};

#[derive(Bundle)]
pub struct PlayerBundle {
	name: Name,
	player: Player,
	rigid_body: RigidBody,
	speed: Speed,
	health: Health,
	scene_bundle: SceneBundle,
	game: OnGameScreen,
	collider: Collider,
	locked_axes: LockedAxes,
	damping: Damping,
	graviy_scale: GravityScale,
	velocity: Velocity,
}

impl PlayerBundle {
	pub fn new(scene: Handle<Scene>) -> Self {
		Self {
			name: Name::new("Player"),
			player: Player::new(100),
			rigid_body: RigidBody::Dynamic,
			speed: Speed(1.),
			health: Health(4.),
			scene_bundle: SceneBundle { scene, ..default() },
			game: OnGameScreen,
			collider: Collider::cylinder(1., 0.75),
			locked_axes: LockedAxes::ROTATION_LOCKED,
			damping: Damping {
				linear_damping: 5.,
				angular_damping: 0.,
			},
			graviy_scale: GravityScale(4.),
			velocity: Velocity::zero(),
		}
	}
}

#[derive(Component)]
pub struct Player {
	money: u32,
}

impl Player {
	pub fn new(money: u32) -> Self {
		Self { money }
	}
	pub fn get_money(&self) -> u32 {
		self.money
	}
	pub fn decrease_money(&mut self, amount: u32) {
		self.money -= amount;
	}
}
