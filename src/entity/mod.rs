mod base;
pub use base::{Base, BaseBundle};

mod player;
pub use player::{Player, PlayerBundle};

mod vehicle;
pub use vehicle::Vehicle;

use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);
