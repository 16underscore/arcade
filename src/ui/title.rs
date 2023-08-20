use bevy::prelude::*;

#[derive(Component)]
enum TitleButtonAction {
	SelectMap,
}

#[derive(Component)]
struct OnTitleScreen;

pub struct TitleScreen;

impl Plugin for TitleScreen {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(super::MenuState::Title), setup_title)
			.add_systems(
				Update,
				title_action.run_if(in_state(super::MenuState::Title)),
			)
			.add_systems(
				OnExit(super::MenuState::Title),
				crate::despawn_screen::<OnTitleScreen>,
			);
	}
}

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
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
			OnTitleScreen,
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
					parent.spawn(ImageBundle {
						style: Style {
							width: Val::Px(384.0),
							height: Val::Px(96.0),
							margin: UiRect::all(Val::Px(50.0)),
							..default()
						},
						image: UiImage::new(asset_server.load("gui/arcade.png")),
						..default()
					});

					parent
						.spawn((
							ButtonBundle {
								style: button_style.clone(),
								background_color: super::GRAY.into(),
								..default()
							},
							TitleButtonAction::SelectMap,
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

fn title_action(
	interaction_query: Query<
		(&Interaction, &TitleButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut menu_state: ResMut<NextState<super::MenuState>>,
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_button_action {
				TitleButtonAction::SelectMap => menu_state.set(super::MenuState::SelectMap),
			}
		}
	}
}
