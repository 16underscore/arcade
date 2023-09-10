mod asset;
mod camera;
mod event;
mod input;
mod shop;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::entity::*;
use crate::map::MapBundle;

use self::asset::{GameAssetPlugin, GameAssets};
use self::camera::Camera3dPlugin;
use self::event::*;
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
			.add_plugins(EventPlugin)
			.add_systems(OnEnter(AppState::Game), setup)
			.add_systems(Update, (in_base, respawn).run_if(in_state(AppState::Game)))
			.add_systems(
				OnExit(AppState::Game),
				super::despawn_screen::<OnGameScreen>,
			);
	}
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>, meshes: Res<Assets<Mesh>>) {
	commands.spawn(MapBundle::new(
		game_assets.scene.map.clone_weak(),
		meshes
			.get(&game_assets.mesh.map)
			.expect("game assets not loaded"),
	));

	for i in 0..4 {
		let position = match i {
			0 => Vec3::new(-30., 0., -30.),
			1 => Vec3::new(-30., 0., 30.),
			2 => Vec3::new(30., 0., -30.),
			3 => Vec3::new(30., 0., 30.),
			_ => Vec3::default(),
		};

		commands.spawn(BaseBundle::new(
			i,
			position,
			game_assets.scene.base.clone_weak(),
		));
	}

	commands.spawn(PlayerBundle::new(game_assets.scene.player.clone_weak()));

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

fn in_base(
	bases: Query<&Name, With<Base>>,
	mut enter_base: EventReader<EnterBaseEvent>,
	mut exit_base: EventReader<ExitBaseEvent>,
) {
	if let Some(enter_base_event) = enter_base.iter().next() {
		match bases.get_component::<Name>(enter_base_event.base_entity) {
			Ok(base_name) => info!("Enter {}", base_name),
			Err(e) => error!("{}", e),
		}
		enter_base.clear();
	}
	if let Some(exit_base_event) = exit_base.iter().next() {
		match bases.get_component::<Name>(exit_base_event.base_entity) {
			Ok(base_name) => info!("Exit {}", base_name),
			Err(e) => error!("{}", e),
		}
		exit_base.clear();
	}
}

fn respawn(
	mut respawn_event: EventReader<RespawnEvent>,
	mut players: Query<&mut Transform, With<Player>>,
) {
	if !respawn_event.is_empty() {
		let mut transform = players.single_mut();
		transform.translation = Vec3::Y;
		respawn_event.clear();
	}
}
