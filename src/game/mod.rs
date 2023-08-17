mod camera;
mod input;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::entity::*;

use self::camera::Camera3dPlugin;
use self::input::InputPlugin;

use super::AppState;

pub struct GamePlugin;

#[derive(Component)]
pub struct OnGameScreen;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
			.add_plugins(Camera3dPlugin)
			.add_plugins(InputPlugin)
			.add_systems(OnEnter(AppState::Game), setup)
			.add_systems(
				OnExit(AppState::Game),
				super::despawn_screen::<OnGameScreen>,
			);
	}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn((
			SceneBundle {
				scene: asset_server.load("map.glb#Scene0"),
				..default()
			},
			OnGameScreen,
		))
		.insert(RigidBody::Fixed)
		.insert(Collider::cuboid(50.0, 0.1, 50.0))
		.insert(Name::new("Ground"));
	commands
		.spawn((
			Player::new(100),
			RigidBody::Dynamic,
			Speed(0.25),
			Health(4.0),
			SceneBundle {
				scene: asset_server.load("player.glb#Scene0"),
				..Default::default()
			},
			OnGameScreen,
		))
		.insert(Collider::cylinder(1.0, 1.0))
		.insert(Name::new("Player"));
	commands
		.spawn((
			RigidBody::Fixed,
			SceneBundle {
				scene: asset_server.load("cannon.glb#Scene0"),
				transform: Transform::from_xyz(10.0, 0.0, 5.0),
				..default()
			},
			OnGameScreen,
		))
		.insert(Collider::cuboid(2.0, 2.0, 2.0))
		.insert(Name::new("Cannon"));

	commands
		.spawn((
			DirectionalLightBundle {
				directional_light: DirectionalLight {
					illuminance: 10000.0,
					shadows_enabled: true,
					..default()
				},
				transform: Transform::from_xyz(-1., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y),
				..default()
			},
			OnGameScreen,
		))
		.insert(Name::new("Sun"));
}
