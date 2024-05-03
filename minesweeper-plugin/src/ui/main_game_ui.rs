use bevy::asset::AssetServer;
// use bevy::prelude::{
// 	default,
// 	AlignItems, BackgroundColor, ButtonBundle, Commands, Component,
// 	Entity, FlexDirection, JustifyContent, JustifyText, NodeBundle, PositionType,
// 	Query, Real, Res, Style, Text, TextBundle, TextSection, TextStyle, Time, Val, With
// };
use bevy::prelude::*;
use crate::AppState;

use crate::components::Flag;
use crate::config::GameConfig;
use crate::events::RestartEvent;
use crate::time::GameTime;


#[derive(Component)]
pub struct TimerUI;

#[derive(Component)]
pub struct FlagsUI;

#[derive(Component)]
pub struct SettingsButton;
#[derive(Component)]
pub struct RestartButton;

pub fn spawn_main_game_ui(
	mut cmd: Commands,
	cfg: Res<GameConfig>,
	srv: Res<AssetServer>,
) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	let text_style = TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.theme.main_ui,
		..default()
	};
	let button_text_style = TextStyle {
		font_size: 20.,
		font: font.clone(),
		color: cfg.theme.background,
		..default()
	};

	let header_box = NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			width: Val::Percent(100.),
			height: Val::Px(cfg.ui_style.header_size.into()),
			top: Val::Percent(0.),
			..default()
		}, ..default()
	};


	let text_box = NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			height: Val::Percent(80.),
			top: Val::Percent(20.),
			width: Val::Percent(50.),
			left: Val::Percent(10.),
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::SpaceEvenly,
			align_items: AlignItems::FlexStart,
			..default()
		}, ..default()
	};

	let button_box = NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			width: Val::Percent(30.),
			height: Val::Percent(70.),
			right: Val::Percent(10.),
			top: Val::Percent(15.),
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::SpaceEvenly,
			align_items: AlignItems::FlexEnd,
			..default()
		}, ..default()
	};

	let flags_box = NodeBundle {
		style: Style {
			height: Val::Percent(40.), ..default()
		}, ..default()
	};
	let flags_text = TextBundle::from_sections([
		TextSection { value: "Flags : ".to_string(), style: text_style.clone() },
		TextSection { value: "0".to_string(), style: text_style.clone() }
	]).with_text_justify(JustifyText::Center);

	let time_box = NodeBundle {
		style: Style {
			height: Val::Percent(40.), ..default()
		}, ..default()
	};
	let time_text = TextBundle::from_sections([
		TextSection { value: "Time : ".to_string(), style: text_style.clone() },
		TextSection { value: "0".to_string(), style: text_style.clone() }
	]).with_text_justify(JustifyText::Center);

	let settings_btn = ButtonBundle {
		style: Style {
			height: Val::Percent(50.),
			width: Val::Percent(80.),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		background_color: BackgroundColor(cfg.theme.main_ui),
		..default()
	};
	let settings_text = TextBundle::from_section("Settings", button_text_style.clone())
		.with_text_justify(JustifyText::Center);

	let restart_btn = ButtonBundle {
		style: Style {
			height: Val::Percent(50.), width: Val::Percent(80.),
			top: Val::Percent(15.),
			justify_content: JustifyContent::Center, align_items: AlignItems::Center,
			..default()
		},
		background_color: BackgroundColor(cfg.theme.main_ui),
		..default()
	};
	let restart_text = TextBundle::from_section("Restart", button_text_style.clone())
		.with_text_justify(JustifyText::Center);

	cmd.spawn(header_box).with_children(|main_box| {
		main_box.spawn(text_box).with_children(|txt_box| {
			txt_box.spawn(flags_box).with_children(|txt| { txt.spawn(flags_text).insert(FlagsUI); });
			txt_box.spawn(time_box).with_children(|txt| { txt.spawn(time_text).insert(TimerUI); });
		});
		main_box.spawn(button_box).with_children(|btn_box| {
			btn_box.spawn(settings_btn).with_children(|btn| { btn.spawn(settings_text); }).insert(SettingsButton);
			btn_box.spawn(restart_btn).with_children(|btn| { btn.spawn(restart_text); }).insert(RestartButton);
		});
	});
}


pub fn update_ui_timer(
	timer: Res<GameTime>,
	mut query: Query<&mut Text, With<TimerUI>>
) {
	if let Ok(mut text) = query.get_single_mut() {
		text.sections[1].value = timer.to_string();
	}
}


pub fn update_ui_flags(
	cfg:   Res<GameConfig>,
	flags: Query<Entity, With<Flag>>,
	mut query: Query<&mut Text, With<FlagsUI>>
) {
	if let Ok(mut text) = query.get_single_mut() {
		let flags_placed = flags.iter().count();
		text.sections[1].value = format!("{} / {}", flags_placed, cfg.difficulty.bomb_count);
	}
}

pub fn settings_button_system(
	mut interaction_q: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>, With<SettingsButton>)>,
	mut next_state: ResMut<NextState<AppState>>,
	cfg: Res<GameConfig>
) {
	for (interaction, mut background) in &mut interaction_q {
		match *interaction {
			Interaction::Pressed => {
				next_state.set(AppState::Settings);
			}
			Interaction::Hovered => {
				background.0 = cfg.theme.main_ui_hover;
			}
			Interaction::None => {
				background.0 = cfg.theme.main_ui;
			}
		}
	}
}

pub fn restart_button_system(
	mut interaction_q: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>, With<RestartButton>),
	>,
	mut ev_restart: EventWriter<RestartEvent>,
	cfg: Res<GameConfig>
) {
	for (interaction, mut background) in &mut interaction_q {
		match *interaction {
			Interaction::Pressed => {
				ev_restart.send(RestartEvent);
			}
			Interaction::Hovered => {
				background.0 = cfg.theme.main_ui_hover;
			}
			Interaction::None => {
				background.0 = cfg.theme.main_ui;
			}
		}
	}
}