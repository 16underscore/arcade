mod player_base;
use std::time::Duration;

use player_base::{base_events, Entered, RunCheck};
pub use player_base::{EnterBaseEvent, ExitBaseEvent, InBaseEvent};

mod respawn;
use respawn::respawn;
pub use respawn::RespawnEvent;

use bevy::prelude::*;

use crate::AppState;

pub struct EventPlugin;

impl Plugin for EventPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(RunCheck {
			timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
		})
		.insert_resource(Entered { base_entity: None })
		.add_event::<EnterBaseEvent>()
		.add_event::<ExitBaseEvent>()
		.add_event::<InBaseEvent>()
		.add_event::<RespawnEvent>()
		.add_systems(
			Update,
			(base_events, respawn).run_if(in_state(AppState::Game)),
		);
	}
}
