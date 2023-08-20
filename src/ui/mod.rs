use bevy::prelude::*;

use super::AppState;

const BACKGROUND_COLOR: Color = Color::rgb(0.0625, 0.0625, 0.0625);
const GRAY: Color = Color::rgb(0.125, 0.125, 0.125);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.125, 0.125, 0.125);
const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = HOVERED_PRESSED_BUTTON;

pub struct MenuPlugin;

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
	Play,
}

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::Menu), setup_ui)
			.add_systems(
				Update,
				(menu_action, button_system).run_if(in_state(AppState::Menu)),
			)
			.add_systems(
				OnExit(AppState::Menu),
				super::despawn_screen::<OnMenuScreen>,
			);
	}
}

fn button_system(
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
		(Changed<Interaction>, With<Button>),
	>,
) {
	for (interaction, mut color, selected) in &mut interaction_query {
		*color = match (*interaction, selected) {
			(Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
			(Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
			(Interaction::Hovered, None) => HOVERED_BUTTON.into(),
			(Interaction::None, None) => NORMAL_BUTTON.into(),
		}
	}
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((Camera2dBundle::default(), OnMenuScreen));
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
				background_color: BACKGROUND_COLOR.into(),
				..default()
			},
			OnMenuScreen,
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
					background_color: GRAY.into(),
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
								background_color: GRAY.into(),
								..default()
							},
							MenuButtonAction::Play,
						))
						.with_children(|parent| {
							parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
						});
				});
		});
}

fn menu_action(
	interaction_query: Query<
		(&Interaction, &MenuButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut app_state: ResMut<NextState<AppState>>,
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_button_action {
				MenuButtonAction::Play => app_state.set(AppState::Game),
			}
		}
	}
}
