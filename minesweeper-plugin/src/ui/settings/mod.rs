mod difficulty;
mod themes;


pub use difficulty::*;

use bevy::prelude::*;

use crate::{AppState, config::GameConfig, ui::GameOverUI};
pub use MainSettings::{
	SettingsDifficulty,
	// SettingsTheme,
	BackToGame
};
use crate::events::UncoverCellEvent;


#[derive(Component)]
pub struct GoToDifficultyMenu;


// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MainSettings {
	SettingsDifficulty,
	// SettingsTheme,
	BackToGame,
}

#[derive(Component)]
pub struct MainSettingsUI;

#[derive(Debug, Copy, Clone, Event)]
pub struct GoToDifficultySettingsEvent;
// #[derive(Debug, Copy, Clone, Event)]
// pub struct GoToThemeSettingsEvent;


// TODO : Rework the entire module to use states and not events to manage settings pages
//


pub fn button_style() -> Style {
	Style {
		width: Val::Px(200.0),
		height: Val::Px(50.0),
		margin: UiRect::all(Val::Px(20.0)),
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		..default()
	}
}

pub fn button_text_style(color: Color, font: Handle<Font>) -> TextStyle {
	TextStyle {
		font_size: 40.0,
		font: font,
		color: color,
	}
}






/// A function that creates the UI for the main Settings Menu
pub fn spawn_settings_ui(mut commands: Commands, cfg: Res<GameConfig>, srv: Res<AssetServer>) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	commands.spawn((
		NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			..default()
		}, MainSettingsUI
	)).with_children(|parent| {
		parent.spawn(NodeBundle {
			style: Style {
				flex_direction: FlexDirection::Column,
				align_items: AlignItems::Center,
				..default()
			},
			background_color: cfg.theme.menus_bg.into(),
			..default()
		}).with_children(|parent| {
			for (action, text) in [
				(MainSettings::SettingsDifficulty, "Difficulty"),
				// (MainSettings::SettingsTheme, "Themes"),
				(MainSettings::BackToGame, "Back"),
			] {
				parent.spawn((
					ButtonBundle {
						style: button_style(),
						background_color: cfg.theme.menus_msg.into(),
						..default()
					},
					action,
				)).with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						text,
						button_text_style(cfg.theme.menus_bg, font.clone()),
					));
				});
			}
		});
	});
}

/// Simple function that despawns the settings menu
pub fn despawn_settings_ui(
	mut cmd: Commands,
	q_ui: Query<Entity, With<MainSettingsUI>>,
) {
	info!("Despawning the main Settings UI...");
	if let Ok(entity) = q_ui.get_single() {
		cmd.entity(entity).despawn_recursive()
	}
}

/// System that handle user interactions with the settings UI
pub fn main_settings_interaction(
	mut q_interaction: Query<
		(&Interaction, &mut BackgroundColor, &MainSettings),
		(Changed<Interaction>, With<Button>)>,
	q_settings_ui: Query<Entity, With<MainSettingsUI>>,
	mut cmd: Commands,
	mut ev_difficulty_settings: EventWriter<GoToDifficultySettingsEvent>,
	// mut ev_theme_settings: EventWriter<GoToThemeSettingsEvent>,
	cfg: Res<GameConfig>,
	mut next_state: ResMut<NextState<AppState>>,
) {
	for (interaction, mut background, action) in &mut q_interaction {
		match *interaction {
			Interaction::Pressed => {
				match *action {
					SettingsDifficulty => {
						if let Ok(ui_entity) = q_settings_ui.get_single() {
							cmd.entity(ui_entity).despawn_recursive();
						};
						ev_difficulty_settings.send(GoToDifficultySettingsEvent);
					},
					// SettingsTheme => {
					// 	if let Ok(ui_entity) = q_settings_ui.get_single() {
					// 		cmd.entity(ui_entity).despawn_recursive();
					// 	};
					// 	ev_theme_settings.send(ThemeSettingsEvent);
					// },
					BackToGame => {
						next_state.set(AppState::InGame);
					},
				}
			}
			Interaction::Hovered => {
				background.0 = cfg.theme.menus_btn;
			}
			Interaction::None => {
				background.0 = cfg.theme.menus_msg;
			}
		}
	}
}

