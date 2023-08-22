mod asset;
mod camera;
mod input;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::entity::*;

use self::asset::{GameAssetPlugin, GameAssets};
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
			.add_plugins(GameAssetPlugin)
			.add_systems(OnEnter(AppState::Game), setup)
			.add_systems(Update, respawn.run_if(in_state(AppState::Game)))
			.add_systems(
				OnExit(AppState::Game),
				super::despawn_screen::<OnGameScreen>,
			);
	}
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>, meshes: Res<Assets<Mesh>>) {
	commands.spawn((
		Name::new("Ground"),
		SceneBundle {
			scene: game_assets.scene.map.clone_weak(),
			..default()
		},
		OnGameScreen,
		RigidBody::Fixed,
		Collider::from_bevy_mesh(
			meshes
				.get(&game_assets.mesh.map)
				.expect("game assets not loaded"),
			&ComputedColliderShape::ConvexHull,
		)
		.expect("meshes not loaded"),
	));

	commands.spawn((
		Name::new("Player"),
		Player::new(100),
		RigidBody::Dynamic,
		Speed(0.25),
		Health(4.),
		SceneBundle {
			scene: game_assets.scene.player.clone_weak(),
			..Default::default()
		},
		OnGameScreen,
		Collider::cylinder(1., 0.75),
		ColliderMassProperties::MassProperties(MassProperties {
			local_center_of_mass: Vec3::new(0., 0.125, 0.),
			mass: 75.,
			..default()
		}),
		Damping {
			linear_damping: 5.,
			angular_damping: 0.,
		},
		GravityScale(4.),
		Velocity::zero(),
	));

	commands.spawn((
		Name::new("Cannon"),
		RigidBody::Fixed,
		SceneBundle {
			scene: game_assets.scene.cannon.clone_weak(),
			transform: Transform::from_xyz(10., 0., 5.),
			..default()
		},
		Collider::cuboid(4., 6., 6.),
		OnGameScreen,
	));

	commands.spawn((
		Name::new("Sun"),
		DirectionalLightBundle {
			directional_light: DirectionalLight {
				illuminance: 10000.,
				shadows_enabled: true,
				..default()
			},
			transform: Transform::from_xyz(-1., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		OnGameScreen,
	));
}

fn respawn(mut players: Query<&mut Transform, With<Player>>) {
	let mut transform = players.single_mut();
	if transform.translation.y < -5. {
		transform.translation = Vec3::new(0., 0., 0.);
	}
}
