use bevy::prelude::*;

use crate::{entity::*, AppState};

pub struct EventPlugin;

impl Plugin for EventPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<PlayerInBaseEvent>()
			.add_event::<RespawnEvent>()
			.add_systems(
				Update,
				(respawn, player_in_base).run_if(in_state(AppState::Game)),
			);
	}
}

#[derive(Event)]
pub struct PlayerInBaseEvent;

#[derive(Event)]
pub struct RespawnEvent;

fn player_in_base(
	mut player_in_base_event: EventWriter<PlayerInBaseEvent>,
	players: Query<&Transform, (With<Player>, Without<Base>)>,
	bases: Query<&Transform, (With<Base>, Without<Player>)>,
) {
	let player_translation = players.single().translation;
	for base_transform in &bases {
		if base_transform
			.translation
			.distance_squared(player_translation)
			< 15f32.powi(2)
		{
			player_in_base_event.send(PlayerInBaseEvent);
		}
	}
}

fn respawn(
	mut respawn_event: EventWriter<RespawnEvent>,
	players: Query<&mut Transform, With<Player>>,
) {
	let transform = players.single();
	if transform.translation.y < -5. {
		respawn_event.send(RespawnEvent);
	}
}
