use bevy::prelude::*;

use crate::entity::*;

#[derive(Event)]
pub struct RespawnEvent;

pub fn respawn(
	mut respawn_event: EventWriter<RespawnEvent>,
	players: Query<&mut Transform, With<Player>>,
) {
	let transform = players.single();
	if transform.translation.y < -5. {
		respawn_event.send(RespawnEvent);
	}
}
