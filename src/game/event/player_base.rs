use bevy::prelude::*;

use crate::entity::*;

#[derive(Event)]
pub struct InBaseEvent {
	pub base_entity: Entity,
}

pub fn in_base_event(
	mut in_base_event: EventWriter<InBaseEvent>,
	players: Query<&Transform, With<Player>>,
	bases: Query<(Entity, &Transform), With<Base>>,
) {
	let player_translation = players.single().translation;
	for (base_entity, base_transform) in &bases {
		let near_base = base_transform
			.translation
			.distance_squared(player_translation)
			< 15f32.powi(2);
		if near_base {
			in_base_event.send(InBaseEvent { base_entity });
		}
	}
}
