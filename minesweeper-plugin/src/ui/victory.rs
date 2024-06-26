use bevy::asset::AssetServer;
use bevy::log::info;
use bevy::prelude::*;
use crate::config::GameConfig;
use crate::events::RestartEvent;
use crate::resources::CoveredCells;

#[derive(Component)]
pub struct VictoryUI;
#[derive(Component)]
pub struct VictoryButtonQuit;
#[derive(Component)]
pub struct VictoryButtonRetry;

pub fn spawn_victory_ui(
	mut cmd: Commands,
	cfg: Res<GameConfig>,
	srv: Res<AssetServer>,
) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	let victory_box = NodeBundle {
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

	let medal_img = ImageBundle {
		style: Style {
			align_self: AlignSelf::Center,
			top: Val::Percent(30.),
			// width: Val::Percent(20.),
			height: Val::Percent(30.),
			..default()
		},
		image: UiImage {
			texture: srv.load("sprites/medal.png"),
			..default()
		},
		..default()
	};


	let victory_text = TextBundle::from_section(
		"Oh woah, You survived !",
		TextStyle {
			font_size: 50.,
			font: font.clone(),
			color: cfg.theme.menus_msg,
			..default()
		},
	).with_text_justify(JustifyText::Center).with_style(
		Style {
			align_self: AlignSelf::Center,
			width: Val::Percent(100.),
			height: Val::Percent(50.),
			position_type: PositionType::Absolute,
			top: Val::Px(5.),
			..default()
		}
	);


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
	let quit_game_text = TextBundle::from_section("Bye bye o/", TextStyle {
		font_size: 25.,
		font: font.clone(),
		color: cfg.theme.menus_bg,
		..default()
	}).with_text_justify(JustifyText::Center);

	cmd.spawn(victory_box).with_children(|main_box| {
		main_box.spawn(medal_img);
		main_box.spawn(victory_text);
		main_box.spawn(button_box).with_children(|btn_box| {
			btn_box.spawn(try_again_btn).with_children(|btn| { btn.spawn(try_again_text); }).insert(VictoryButtonRetry);
			btn_box.spawn(quit_game_btn).with_children(|btn| { btn.spawn(quit_game_text); }).insert(VictoryButtonQuit);
		});

	}).insert(VictoryUI);
}

pub fn despawn_victory_ui(
	mut cmd: Commands,
	q_ui: Query<Entity, With<VictoryUI>>,
) {
	info!("Despawning the Game Over UI...");
	if let Ok(entity) = q_ui.get_single() {
		cmd.entity(entity).despawn_recursive()
	}
}


pub fn victory_quit_button(
	mut q_interaction: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>, With<VictoryButtonQuit>),
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

pub fn victory_restart_button(
	mut q_interaction: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>, With<VictoryButtonRetry>),
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

