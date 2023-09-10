use bevy::prelude::*;

use crate::entity::*;

#[derive(Resource)]
pub struct RunCheck {
	pub timer: Timer,
}

#[derive(Resource)]
pub struct Entered {
	pub base_entity: Option<Entity>,
}

#[derive(Event)]
pub struct InBaseEvent {
	pub base_entity: Entity,
}

#[derive(Event)]
pub struct EnterBaseEvent {
	pub base_entity: Entity,
}

#[derive(Event)]
pub struct ExitBaseEvent {
	pub base_entity: Entity,
}

pub fn base_events(
	time: Res<Time>,
	mut run_check: ResMut<RunCheck>,
	mut entered: ResMut<Entered>,
	mut in_base_event: EventWriter<InBaseEvent>,
	mut enter_base_event: EventWriter<EnterBaseEvent>,
	mut exit_base_event: EventWriter<ExitBaseEvent>,
	players: Query<&Transform, With<Player>>,
	bases: Query<(Entity, &Transform), With<Base>>,
) {
	run_check.timer.tick(time.delta());
	if !run_check.timer.just_finished() {
		return;
	}
	let player_translation = players.single().translation;
	for (base_entity, base_transform) in &bases {
		let near_base = base_transform
			.translation
			.distance_squared(player_translation)
			< 15f32.powi(2);
		if near_base {
			in_base_event.send(InBaseEvent { base_entity });
			if let None = entered.base_entity {
				enter_base_event.send(EnterBaseEvent { base_entity });
				entered.base_entity = Some(base_entity);
			}
		} else {
			if entered.base_entity == Some(base_entity) {
				exit_base_event.send(ExitBaseEvent { base_entity });
				entered.base_entity = None;
			}
		}
	}
}
