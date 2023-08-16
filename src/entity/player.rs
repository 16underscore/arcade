use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
	money: u32,
}

impl Player {
	pub fn new(money: u32) -> Self {
		Self { money }
	}
	pub fn get_money(&self) -> u32 {
		self.money
	}
	pub fn decrease_money(&mut self, amount: u32) {
		self.money -= amount;
	}
}
