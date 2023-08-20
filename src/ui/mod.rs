mod select_map;
mod title;

use bevy::prelude::*;

use self::select_map::SelectMapScreen;
use self::title::TitleScreen;

use super::AppState;

const BACKGROUND_COLOR: Color = Color::rgb(0.0625, 0.0625, 0.0625);
const GRAY: Color = Color::rgb(0.125, 0.125, 0.125);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.125, 0.125, 0.125);
const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = HOVERED_PRESSED_BUTTON;

#[derive(Component)]
struct OnMenuScreen;

pub struct MenuPlugin;

#[derive(Component)]
struct SelectedOption;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum MenuState {
	None,
	#[default]
	Title,
	SelectMap,
}

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(TitleScreen)
			.add_plugins(SelectMapScreen)
			.add_state::<MenuState>()
			.add_systems(OnEnter(super::AppState::Menu), setup_ui)
			.add_systems(Update, button_system.run_if(in_state(AppState::Menu)))
			.add_systems(
				OnExit(super::AppState::Menu),
				crate::despawn_screen::<OnMenuScreen>,
			);
	}
}

fn setup_ui(mut commands: Commands) {
	commands.spawn((Camera2dBundle::default(), OnMenuScreen));
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
