use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::ui::UiImage;

use crate::config::GameConfig;
use crate::events::RestartEvent;

#[derive(Component)]
pub struct GameOverUI;
#[derive(Component)]
pub struct ButtonQuit;
#[derive(Component)]
pub struct ButtonRetry;

pub fn spawn_game_over_ui(
	mut cmd: Commands,
		cfg: Res<GameConfig>,
		srv: Res<AssetServer>,
) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	let gameover_box = NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(40.),
			top: Val::Percent(40.),
			flex_direction: FlexDirection::Column,
			..default()
		},
		background_color: BackgroundColor(cfg.theme.menus_bg),
		..default()
	};

	let skull_img = ImageBundle {
		style: Style {
			align_self: AlignSelf::Center,
			top: Val::Percent(30.),
			// width: Val::Percent(20.),
			height: Val::Percent(30.),
			..default()
		},
		image: UiImage {
			texture: srv.load("sprites/skull.png"),
			..default()
		},
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
			color: cfg.theme.menus_msg,
			..default()
		},
	).with_text_justify(JustifyText::Center);


	let button_box = NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Row,
			width: Val::Percent(100.),
			height: Val::Percent(20.),
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
		background_color: BackgroundColor(cfg.theme.menus_msg),
		..default()
	};
	let try_again_text = TextBundle::from_section("Wanna try again ?", TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.theme.menus_bg,
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
		background_color: BackgroundColor(cfg.theme.menus_msg),
		..default()
	};
	let quit_game_text = TextBundle::from_section("Hell nah, get me out", TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.theme.menus_bg,
		..default()
	}).with_text_justify(JustifyText::Center);

	cmd.spawn(gameover_box).with_children(|main_box| {
		main_box.spawn(skull_img);
		main_box.spawn(text_box).with_children(|txt_box| {
			txt_box.spawn(gameover_text);
		});
		main_box.spawn(button_box).with_children(|btn_box| {
			btn_box.spawn(try_again_btn).with_children(|btn| { btn.spawn(try_again_text); }).insert(ButtonRetry);
			btn_box.spawn(quit_game_btn).with_children(|btn| { btn.spawn(quit_game_text); }).insert(ButtonQuit);
		});

	}).insert(GameOverUI);
}

pub fn despawn_game_over_ui(
	mut cmd: Commands,
	q_ui: Query<Entity, With<GameOverUI>>,
) {
	info!("Despawning the Game Over UI...");
	if let Ok(entity) = q_ui.get_single() {
		cmd.entity(entity).despawn_recursive()
	}
}


pub fn quit_button_system(
	mut q_interaction: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>, With<ButtonQuit>),
	>,
	mut ev_quit_game: ResMut<Events<bevy::app::AppExit>>,
		cfg: Res<GameConfig>,
) {
	for (interaction, mut background) in &mut q_interaction {
		match *interaction {
			Interaction::Pressed => {
				ev_quit_game.send(bevy::app::AppExit);
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

pub fn retry_button_system(
	mut q_interaction: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>, With<ButtonRetry>),
	>,
	mut ev_restart: EventWriter<RestartEvent>,
		cfg: Res<GameConfig>,
) {
	for (interaction, mut background) in &mut q_interaction {
		match *interaction {
			Interaction::Pressed => {
				ev_restart.send(RestartEvent);
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

