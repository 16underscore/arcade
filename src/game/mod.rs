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
	commands.spawn((
		Name::new("Ground"),
		SceneBundle {
			scene: asset_server.load("map/map.glb#Scene0"),
			..default()
		},
		OnGameScreen,
		RigidBody::Fixed,
		Collider::cuboid(50.0, 0.1, 100.0),
	));

	commands.spawn((
		Name::new("Player"),
		Player::new(100),
		RigidBody::Dynamic,
		Speed(0.25),
		Health(4.0),
		SceneBundle {
			scene: asset_server.load("entity/player.glb#Scene0"),
			..Default::default()
		},
		OnGameScreen,
		Collider::cylinder(1.0, 0.75),
		ColliderMassProperties::MassProperties(MassProperties {
			local_center_of_mass: Vec3::new(0., 0.125, 0.),
			mass: 75.0,
			..default()
		}),
		Damping {
			linear_damping: 5.0,
			angular_damping: 0.,
		},
		GravityScale(2.0),
		Velocity::zero(),
	));

	commands.spawn((
		Name::new("Cannon"),
		RigidBody::Fixed,
		SceneBundle {
			scene: asset_server.load("entity/cannon.glb#Scene0"),
			transform: Transform::from_xyz(10.0, 0.0, 5.0),
			..default()
		},
		Collider::cuboid(4.0, 6.0, 6.0),
		OnGameScreen,
	));

	commands.spawn((
		Name::new("Sun"),
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
	));
}
