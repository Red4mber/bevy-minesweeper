use bevy::prelude::*;
use crate::AppState;
use crate::config::{GameConfig, DifficultySettings};
use crate::events::{RestartEvent, ChangeDifficultyEvent};
use crate::ui::{GoToDifficultySettingsEvent, MainSettingsUI};
use super::{
    MainSettings,
	BackToGame,
    SettingsDifficulty,
    // SettingsTheme
};
use super::{button_style, button_text_style};



#[derive(Component)]
pub struct DifficultySettingsUI;

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum DifficultyUIButtons {
	Beginner,
	Intermediate,
	Expert,
	BackToSettings,
}


pub fn spawn_difficulty_settings_ui(mut commands: Commands, cfg: Res<GameConfig>, srv: Res<AssetServer>) {
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
		}, DifficultySettingsUI
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
				(DifficultyUIButtons::Beginner, "Beginner"),
				(DifficultyUIButtons::Intermediate, "Intermediate"),
				(DifficultyUIButtons::Expert, "Expert"),
				(DifficultyUIButtons::BackToSettings, "<== Back"),
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
pub fn despawn_difficulty_settings_ui(
	mut cmd: Commands,
	q_ui: Query<Entity, With<MainSettingsUI>>,
) {
	info!("Despawning the main Settings UI...");
	if let Ok(entity) = q_ui.get_single() {
		cmd.entity(entity).despawn_recursive()
	}
}



pub fn difficulty_settings_interaction(
	mut q_interaction: Query<(&Interaction, &mut BackgroundColor, &DifficultyUIButtons), (Changed<Interaction>, With<Button>)>,
		mut q_ui: Query<Entity, With<DifficultySettingsUI>>,
	mut ev_change_difficulty: EventWriter<ChangeDifficultyEvent>,
	mut next_state: ResMut<NextState<AppState>>,
	mut cmd: Commands,
	cfg: Res<GameConfig>,
) {
	for (interaction, mut background, action) in &mut q_interaction {
		match *interaction {
			Interaction::Pressed => {
				if let Ok(entity) = q_ui.get_single_mut() {
					cmd.entity(entity).despawn_recursive()
				}
				match *action {
					DifficultyUIButtons::Beginner => {
						ev_change_difficulty.send(
							ChangeDifficultyEvent(DifficultySettings::beginner())
						);
					},
					DifficultyUIButtons::Intermediate => {
						ev_change_difficulty.send(
							ChangeDifficultyEvent(DifficultySettings::intermediate())
						);
					},
					DifficultyUIButtons::Expert => {

						ev_change_difficulty.send(
							ChangeDifficultyEvent(DifficultySettings::expert())
						);
					},
					DifficultyUIButtons::BackToSettings=> {
						next_state.set(AppState::InGame);  // Reloading the Settings App State to spawn the settings menu again
						next_state.set(AppState::Settings);// A bit hacky but it works so who cares
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
