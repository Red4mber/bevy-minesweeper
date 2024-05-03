extern crate core;


use std::ops::DerefMut;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy::window::WindowResolution;


mod config;
mod components;
mod resources;
mod input;
mod events;
mod ui;
mod time;




use crate::{
	input::mouse_button_events,
	config::{
		ColorTheme, GameConfig
	},
	ui::{
		MainSettings, GoToDifficultySettingsEvent, //GoToThemeSettingsEvent,
		spawn_game_over_ui, spawn_main_game_ui, spawn_settings_ui, spawn_difficulty_settings_ui, spawn_victory_ui,
		despawn_game_over_ui, despawn_victory_ui, despawn_settings_ui,
		update_ui_timer, update_ui_flags,
		settings_button_system, restart_button_system, retry_button_system, quit_button_system,
		victory_quit_button, victory_restart_button,
		main_settings_interaction, difficulty_settings_interaction,

	},
	events::{
		ExplosionEvent, UncoverCellEvent, FlagCellEvent, RestartEvent, ChangeDifficultyEvent, //VictoryEvent,
		uncover_cell, despawn_grid, update_difficulty,
		explosion_event_handler, flag_event_handler, uncover_event_handler,
	},
	components::{
		Bomb, BombNeighbors, EmptyCell, Coordinates, GridContainer
	},
	resources::{
		CoveredCells, Grid, Cell,
	},
	time::{
		GameTime,
		update_timer, pause_timer, unpause_timer, restart_timer,
	}
};


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
	Settings,
	#[default]
	InGame,
	GameOver,
	Victory,
}

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ColorTheme>()
			.init_resource::<GameConfig>()
			.init_resource::<GameTime>()
			.init_resource::<CoveredCells>()
			.add_event::<UncoverCellEvent>()
			.add_event::<FlagCellEvent>()
			.add_event::<RestartEvent>()
			// .add_event::<VictoryEvent>()
			.add_event::<ExplosionEvent>()
			// .add_event::<MainSettings>()
			.add_event::<GoToDifficultySettingsEvent>()
			.add_event::<ChangeDifficultyEvent>()
			// .add_event::<GoToThemeSettingsEvent>()
			.insert_state(AppState::InGame)
			.add_systems(Startup, (update_window, spawn_camera, game_setup, spawn_main_game_ui))
			// .add_systems(PostStartup, unpause_gametimer)
			.add_systems(Update, (
				update_timer,
				(
					uncover_cell, update_ui_timer, update_ui_flags,
					restart_button_system, settings_button_system,
					mouse_button_events
				).run_if(in_state(AppState::InGame)),
				(
					quit_button_system, retry_button_system
				).run_if(in_state(AppState::GameOver)),
				(
					victory_quit_button, victory_restart_button
				).run_if(in_state(AppState::Victory)),
				(
					main_settings_interaction, difficulty_settings_interaction,
				).run_if(in_state(AppState::Settings)),
				(
					despawn_grid,
					game_setup.after(despawn_grid),
					(update_window, update_camera, restart_timer).after(game_setup),
				).run_if(on_event::<RestartEvent>()),
				// spawn_victory_ui            .run_if(on_event::<VictoryEvent>()),
				uncover_event_handler       .run_if(on_event::<UncoverCellEvent>()),
				explosion_event_handler     .run_if(on_event::<ExplosionEvent>()),
				flag_event_handler          .run_if(on_event::<FlagCellEvent>()),
				spawn_difficulty_settings_ui.run_if(on_event::<GoToDifficultySettingsEvent>()),
				update_difficulty           .run_if(on_event::<ChangeDifficultyEvent>())

		   ))
			.add_systems(OnEnter(AppState::GameOver), spawn_game_over_ui)
			.add_systems(OnExit(AppState::GameOver), despawn_game_over_ui)
			.add_systems(OnEnter(AppState::Victory), spawn_victory_ui)
			.add_systems(OnExit(AppState::Victory), despawn_victory_ui)
			.add_systems(OnExit(AppState::InGame), pause_timer)
			.add_systems(OnEnter(AppState::InGame), unpause_timer)
			.add_systems(OnEnter(AppState::Settings), spawn_settings_ui)
			.add_systems(OnExit(AppState::Settings), despawn_settings_ui);
	}
}


fn game_setup(
	mut cmd: Commands,
	cfg: Res<GameConfig>,
	srv: Res<AssetServer>,
	mut covers_res: ResMut<CoveredCells>,
	mut next_state: ResMut<NextState<AppState>>,

) {
	let font = srv.load("fonts/FiraSans-Black.ttf");
	let bomb_sprite = srv.load("sprites/mine.png");

	let mut cell_covers: HashMap<Coordinates, Entity> = HashMap::new();

	let mut grid = Grid::empty(cfg.difficulty.grid_size.x, cfg.difficulty.grid_size.y);
	grid.set_bombs(cfg.difficulty.bomb_count);

	debug!("{}", grid.console_output());

	let grid_size = <Coordinates as Into<Vec2>>::into(cfg.difficulty.grid_size)
		* Vec2::splat(cfg.ui_style.cell_size.into())
		+ Vec2::splat(cfg.ui_style.cell_padding.into());

	let grid_position: Vec3 = Vec2::splat(cfg.ui_style.margin.into()).extend(0.);

	// Spawning the grid background as a parent container
	cmd.spawn(SpriteBundle {
			transform: Transform::from_translation(grid_position),
			sprite: Sprite {
				color: cfg.theme.border,
				custom_size: Some(grid_size),
				anchor: Anchor::BottomLeft,
				..default()
			},
			..default()
		}).insert(GridContainer).insert(Name::new("Grid Background"))
		// Spawning every cells
		.with_children(|parent| {
			for (row, line) in grid.iter().enumerate() {
				for (col, cell) in line.iter().enumerate() {
					let mut cmd = parent.spawn_empty();
					// Spawns the cell background
					cmd.insert(SpriteBundle {
						sprite: Sprite {
							color: cfg.theme.cell,
							custom_size: Some(Vec2::splat(
								(cfg.ui_style.cell_size - cfg.ui_style.cell_padding) as f32,
							)),
							anchor: Anchor::BottomLeft,
							..default()
						},
						transform: Transform::from_xyz(
							((col * cfg.ui_style.cell_size as usize) + cfg.ui_style.cell_padding as usize) as f32,
							((row * cfg.ui_style.cell_size as usize) + cfg.ui_style.cell_padding as usize) as f32,
							1.,
						),
						..Default::default()
					}).insert(Name::new(format!("Cell {col} - {row}")))
					  .insert(Coordinates { x: col as u16, y: row as u16 });

					// Covering the cell
					cmd.with_children(|parent| {
						let entity = parent.spawn(SpriteBundle {
							sprite: Sprite {
								color: cfg.theme.hidden_cell,
								custom_size: Some(Vec2::splat(
									(cfg.ui_style.cell_size - cfg.ui_style.cell_padding) as f32,
								)),
								anchor: Anchor::BottomLeft,
								..default()
							},
							transform: Transform::from_xyz( 0.,0.,2.),
							..Default::default()
						}).insert(Name::new("Cell Cover")).id();
						cell_covers.insert(Coordinates {x: col as u16, y: row as u16}, entity);
					});

					// Adding the cell's specific information depending on it's type
					match cell {
						Cell::Empty => {
							cmd.insert(EmptyCell);
						},
						Cell::Bomb => {
							cmd.insert(Bomb);
							cmd.with_children(|parent| {
								parent.spawn(SpriteBundle {
									sprite: Sprite {
										custom_size: Some(Vec2::splat((cfg.ui_style.cell_size - cfg.ui_style.cell_padding - 2) as f32)),
										..default()
									},
									transform: Transform::from_xyz(
										cfg.ui_style.cell_size as f32 / 2.,
										cfg.ui_style.cell_size as f32 / 2.,
										1.
									),
									texture: bomb_sprite.clone(),
									..default()
								}).insert(Name::new("Bomb Sprite"));
							});
						},
						Cell::BombNeighbor(count) => {
							cmd.insert(BombNeighbors { count: *count });
							cmd.with_children(|parent| {
								parent.spawn(Text2dBundle {
									text: Text::from_section((*count).to_string(), TextStyle {
										color: cfg.theme.neighbors[(*count - 1) as usize],
										font: font.clone(),
										font_size: cfg.ui_style.font_size as f32,
									}),
									transform: Transform::from_xyz(
										cfg.ui_style.cell_size as f32 / 2.,
										cfg.ui_style.cell_size as f32 / 2.,
										1.
									),
									..default()
								}).insert(Name::new("Neighbor count"));
							});
						}
					}

				}
			}
		});

	*covers_res = CoveredCells(cell_covers);
	next_state.set(AppState::InGame);
}

#[derive(Component)]
pub struct GameCamera;

fn spawn_camera(mut cmd: Commands, cfg: Res<GameConfig>) {
	let window_size = <Coordinates as Into<Vec2>>::into(cfg.difficulty.grid_size) * Vec2::splat(cfg.ui_style.cell_size.into())
		+ Vec2::splat(cfg.ui_style.margin as f32 * 2.) + Vec2::new(0., cfg.ui_style.header_size.into());
	cmd.spawn((GameCamera, Camera2dBundle {
		transform: Transform::from_translation((window_size/2.).extend(0.)),
		camera: Camera {
			clear_color: ClearColorConfig::from(cfg.theme.background),
			..default()
		},
		..default()
	}));
}
fn update_camera(mut q_camera: Query<(&Camera, &mut Transform), With<GameCamera>>, mut cmd: Commands, cfg: Res<GameConfig>) {
	let window_size = <Coordinates as Into<Vec2>>::into(cfg.difficulty.grid_size) * Vec2::splat(cfg.ui_style.cell_size.into())
		+ Vec2::splat(cfg.ui_style.margin as f32 * 2.) + Vec2::new(0., cfg.ui_style.header_size.into());
	if let Ok((_, mut transform)) = q_camera.get_single_mut() {
		*transform = Transform::from_translation((window_size/2.).extend(0.));
	}
}
fn update_window(mut window: Query<&mut Window>, cfg: Res<GameConfig>) {
	// Resize the window to fit the grid
	let window_size = <Coordinates as Into<Vec2>>::into(cfg.difficulty.grid_size) * Vec2::splat(cfg.ui_style.cell_size.into())
		+ Vec2::splat(cfg.ui_style.margin as f32 * 2.) + Vec2::new(0., cfg.ui_style.header_size.into());

	window.single_mut().resolution = WindowResolution::from(window_size);

	window.single_mut().visible = true;
}

