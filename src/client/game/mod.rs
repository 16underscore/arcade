use bevy::{prelude::*, window::PrimaryWindow};

use crate::entity::{Health, Player, Speed, Vehicle};

use super::AppState;

pub struct GamePlugin;

#[derive(Component)]
struct OnGameScreen;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::Game), setup)
			.add_systems(Update, input.run_if(in_state(AppState::Game)))
			.add_systems(Update, summon_vehicle.run_if(in_state(AppState::Game)))
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
		.insert(Name::new("Ground"));
	commands
		.spawn((
			Player::new(100),
			Speed(0.25),
			Health(4.0),
			SceneBundle {
				scene: asset_server.load("player.glb#Scene0"),
				..Default::default()
			},
			OnGameScreen,
		))
		.insert(Name::new("Player"));
	commands
		.spawn((
			SceneBundle {
				scene: asset_server.load("cannon.glb#Scene0"),
				transform: Transform::from_xyz(10.0, 0.0, 5.0),
				..default()
			},
			OnGameScreen,
		))
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
	commands
		.spawn((
			Camera3dBundle {
				transform: Transform::from_xyz(-20.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
				..default()
			},
			OnGameScreen,
		))
		.insert(Name::new("Camera"));
}

fn input(
	gamepads: Res<Gamepads>,
	axes: Res<Axis<GamepadAxis>>,
	windows: Query<&Window, With<PrimaryWindow>>,
	keyboard: Res<Input<KeyCode>>,
	mut transforms: Query<&mut Transform, Or<(With<Player>, With<Camera3d>)>>,
	speeds: Query<&Speed, With<Player>>,
) {
	let window = windows.single();
	let Speed(speed) = speeds.single();
	if let Some(position) = window.cursor_position() {
		let halfwidth = window.width() / 2.0;
		let halfheight = window.height() / 2.0;
		let horizontal = (position.x - halfwidth) / halfwidth;
		let vertical = (position.y - halfheight) / halfheight;
		let (x, z) = calc(vertical, horizontal, 4.0);
		for mut transform in &mut transforms {
			if keyboard.pressed(KeyCode::W) {
				transform.translation.x -= x * speed;
				transform.translation.z += z * speed;
			}
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
		for mut transform in &mut transforms {
			transform.translation.x += x * speed;
			transform.translation.z += z * speed;
		}
	}
}

fn summon_vehicle(
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	keyboard: Res<Input<KeyCode>>,
	mut players: Query<(&mut Player, &Transform)>,
) {
	let (mut player, transform) = players.single_mut();
	if keyboard.just_pressed(KeyCode::Space) {
		if player.get_money() >= 20 {
			player.decrease_money(20);
			commands
				.spawn((
					Vehicle,
					Health(40.0),
					Speed(0.25),
					SceneBundle {
						scene: asset_server.load("tank.glb#Scene0"),
						transform: transform.clone(),
						..default()
					},
					OnGameScreen,
				))
				.insert(Name::new("Tank"));
		}
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
