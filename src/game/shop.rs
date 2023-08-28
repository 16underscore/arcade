use bevy::prelude::*;

use super::OnGameScreen;

#[derive(Component)]
pub struct Shop;

pub fn setup(commands: &mut Commands) {
	commands
		.spawn((
			Name::new("Shop"),
			Shop,
			OnGameScreen,
			NodeBundle {
				style: Style {
					width: Val::Percent(100.),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(NodeBundle {
				style: Style {
					width: Val::Percent(30.),
					height: Val::Percent(30.),
					..default()
				},
				background_color: Color::rgba(0.0625, 0.0625, 0.0625, 0.75).into(),
				..default()
			});
		});
}
