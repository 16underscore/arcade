mod player_base;
use player_base::base_events;
pub use player_base::InBaseEvent;

mod respawn;
use respawn::respawn;
pub use respawn::RespawnEvent;

use bevy::prelude::*;

use crate::AppState;

pub struct EventPlugin;

impl Plugin for EventPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<InBaseEvent>()
			.add_event::<RespawnEvent>()
			.add_systems(
				Update,
				(base_events, respawn).run_if(in_state(AppState::Game)),
			);
	}
}
