use bevy::prelude::*;

#[derive(Component)]
enum SelectMapButtonAction {
	Select,
}

#[derive(Component)]
struct OnSelectMapScreen;

pub struct SelectMapScreen;

impl Plugin for SelectMapScreen {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(super::MenuState::SelectMap), setup_select_map)
			.add_systems(
				Update,
				select_map_action.run_if(in_state(super::MenuState::SelectMap)),
			)
			.add_systems(
				OnExit(super::MenuState::SelectMap),
				crate::despawn_screen::<OnSelectMapScreen>,
			);
	}
}

fn setup_select_map(mut commands: Commands) {
	let button_style = Style {
		width: Val::Px(255.0),
		height: Val::Px(63.0),
		margin: UiRect::all(Val::Px(31.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	};
	let button_text_style = TextStyle {
		font_size: 31.0,
		color: Color::GRAY,
		..default()
	};

	commands
		.spawn((
			NodeBundle {
				style: Style {
					width: Val::Percent(100.0),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				background_color: super::BACKGROUND_COLOR.into(),
				..default()
			},
			OnSelectMapScreen,
		))
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: Style {
						flex_direction: FlexDirection::Column,
						align_items: AlignItems::Center,
						padding: UiRect::horizontal(Val::Px(100.0)),
						..default()
					},
					background_color: super::GRAY.into(),
					..default()
				})
				.with_children(|parent| {
					parent
						.spawn((
							ButtonBundle {
								style: button_style.clone(),
								background_color: super::GRAY.into(),
								..default()
							},
							SelectMapButtonAction::Select,
						))
						.with_children(|parent| {
							parent.spawn(TextBundle::from_section(
								"Select Map",
								button_text_style.clone(),
							));
						});
				});
		});
}

fn select_map_action(
	interaction_query: Query<
		(&Interaction, &SelectMapButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut app_state: ResMut<NextState<crate::AppState>>,
	mut menu_state: ResMut<NextState<super::MenuState>>,
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_button_action {
				SelectMapButtonAction::Select => {
					menu_state.set(super::MenuState::None);
					app_state.set(crate::AppState::Game);
				}
			}
		}
	}
}
