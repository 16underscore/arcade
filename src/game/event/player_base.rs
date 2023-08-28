use bevy::prelude::*;

use crate::entity::*;

#[derive(Event)]
pub struct InBaseEvent {
	pub base_entity: Entity,
}

pub fn base_events(
	mut enter_base_event: EventWriter<InBaseEvent>,
	players: Query<&Transform, (With<Player>, Without<Base>)>,
	bases: Query<(Entity, &Transform), (With<Base>, Without<Player>)>,
) {
	let player_translation = players.single().translation;
	for (base_entity, base_transform) in &bases {
		let near_base = base_transform
			.translation
			.distance_squared(player_translation)
			< 15f32.powi(2);
		if near_base {
			enter_base_event.send(InBaseEvent { base_entity });
		}
	}
}
