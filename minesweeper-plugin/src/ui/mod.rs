use bevy::prelude::*;
use bevy_inspector_egui::egui::Align;
use crate::components::Flag;

// use crate::GameTimer;
use crate::config::GameConfig;





pub fn spawn_game_over_ui(
	mut cmd: Commands,
		cfg: Res<GameConfig>,
		srv: Res<AssetServer>,
) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	let gameover_box = NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(20.),
			top: Val::Percent(40.),
			flex_direction: FlexDirection::Column,
			..default()
		},
		background_color: BackgroundColor(cfg.colors.gameover_box),
		..default()
	};
	let text_box = NodeBundle {
		style: Style {
			justify_content: JustifyContent::Center,
			width: Val::Percent(100.),
			height: Val::Percent(50.),
			position_type: PositionType::Absolute,
			top: Val::Px(5.),
			..default()
		},
		..default()
	};

	let gameover_text = TextBundle::from_section(
		"Oh shit, you blew up !",
		TextStyle {
			font_size: 50.,
			font: font.clone(),
			color: cfg.colors.gameover_msg,
			..default()
		},
	).with_text_justify(JustifyText::Center);


	let button_box = NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Row,
			width: Val::Percent(100.),
			height: Val::Percent(30.),
			position_type: PositionType::Absolute,
			bottom: Val::Percent(10.),
			..default()
		},
		// background_color: BackgroundColor(Color::GREEN),
		..default()
	};
	let try_again_btn = ButtonBundle {
		style: Style {
			height: Val::Percent(100.),
			width: Val::Percent(40.),
			left: Val::Percent(5.),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		background_color: BackgroundColor(cfg.colors.gameover_msg),
		..default()
	};
	let try_again_text = TextBundle::from_section("Wanna try again ?", TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.colors.gameover_box,
		..default()
	}).with_text_justify(JustifyText::Center);

	let quit_game_btn = ButtonBundle {
		style: Style {
			height: Val::Percent(100.),
			width: Val::Percent(40.),
			position_type: PositionType::Absolute,
			flex_direction: FlexDirection::Column,
			right: Val::Percent(5.),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		background_color: BackgroundColor(cfg.colors.gameover_msg),
		..default()
	};
	let quit_game_text = TextBundle::from_section("Hell nah, get me out", TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.colors.gameover_box,
		..default()
	}).with_text_justify(JustifyText::Center);

	cmd.spawn(gameover_box).with_children(|main_box| {
		main_box.spawn(text_box).with_children(|txt_box| {
			txt_box.spawn(gameover_text);
		});
		main_box.spawn(button_box).with_children(|btn_box| {
			btn_box.spawn(try_again_btn).with_children(|btn| { btn.spawn(try_again_text); });
			btn_box.spawn(quit_game_btn).with_children(|btn| { btn.spawn(quit_game_text); });
		});

	});
}


#[derive(Component)]
pub struct TimerUI;

#[derive(Component)]
pub struct FlagsUI;


pub fn spawn_main_ui(
	mut cmd: Commands,
	cfg: Res<GameConfig>,
	srv: Res<AssetServer>,
) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	let text_style = TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.colors.gameover_msg,
		..default()
	};
	let button_text_style = TextStyle {
		font_size: 20.,
		font: font.clone(),
		color: cfg.colors.background,
		..default()
	};

	let header_box = NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			width: Val::Percent(100.),
			height: Val::Px(cfg.header_size.into()),
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
		background_color: BackgroundColor(cfg.colors.gameover_msg),
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
		background_color: BackgroundColor(cfg.colors.gameover_msg),
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
			btn_box.spawn(settings_btn).with_children(|btn| { btn.spawn(settings_text); });
			btn_box.spawn(restart_btn).with_children(|btn| { btn.spawn(restart_text); });
		});
	});
}

pub fn update_ui_timer(
		timer: Res<Time<Real>>,
	mut query: Query<&mut Text, With<TimerUI>>
) {
	if let Ok(mut text) = query.get_single_mut() {
		let elapsed_secs = timer.elapsed_seconds();
		text.sections[1].value = if elapsed_secs > 60. {
			format!("{} mins {} secs", (elapsed_secs / 60.).trunc(), (elapsed_secs % 60.).trunc())
		} else {
			format!("{} secs", elapsed_secs.trunc())
		};
	}
}

pub fn update_ui_flags(
		cfg:   Res<GameConfig>,
		timer: Res<Time>,
		flags: Query<Entity, With<Flag>>,
	mut query: Query<&mut Text, With<FlagsUI>>
) {
	if let Ok(mut text) = query.get_single_mut() {
		let flags_placed = flags.iter().count();
		text.sections[1].value = format!("{} / {}", flags_placed, cfg.bomb_count);
	}
}