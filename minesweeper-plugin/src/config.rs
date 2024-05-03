use bevy::prelude::*;
use crate::components::Coordinates;

// This file contains the default configuration for the game

/// Structure containing all the colors used throughout the game, allowing easy customization
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ColorTheme {
	pub background: Color,
	pub border: Color,
	pub cell: Color,
	pub hidden_cell: Color,
	pub main_ui: Color,
	pub main_ui_hover: Color,
	pub menus_bg: Color,
	pub menus_msg: Color,
	pub menus_btn: Color,
	pub flags: Color,
	pub neighbors: [Color; 8],
}

impl Default for ColorTheme {
	fn default() -> Self {
		Self {
			background:     Color::hex("203c56").unwrap(),
			border:         Color::hex("738eab").unwrap(),
			cell:           Color::hex("4d6285").unwrap(),
			hidden_cell:    Color::hex("303b69").unwrap(),
			flags:          Color::hex("d08159").unwrap(),
			main_ui:        Color::hex("738eab").unwrap(),
			main_ui_hover:  Color::hex("536e8b").unwrap(),
			menus_bg:       Color::hex("7c183c").unwrap(),
			menus_btn:      Color::hex("df7264").unwrap(),
			menus_msg:      Color::hex("ff8274").unwrap(),
			neighbors:  [
				Color::hex("6d85a5").unwrap(),
				Color::hex("6cb9c9").unwrap(),
				Color::hex("9ceded").unwrap(),
				Color::hex("ae8181").unwrap(),
				Color::hex("bf1d5c").unwrap(),
				Color::hex("4f1446").unwrap(),
				Color::hex("2e0a30").unwrap(),
				Color::hex("0d001a").unwrap()
			]
		}
	}
}

// Simple type alias to have the handy Coordinates struct without naming it ^^
type GridSize = Coordinates;

/// Structure containing the two parameters influencing the difficulty of a minesweeper game,
/// namely the grid size (width and height) and bomb count
///
/// it implements defaults for the three standard minesweeper difficulty settings
#[derive(Resource, Reflect, Copy, Clone)]
#[reflect(Resource)]
pub struct DifficultySettings {
	pub grid_size: GridSize, // (u16, u16),
	pub bomb_count: u16,
}
impl DifficultySettings {
	pub fn beginner() -> Self {
		Self {
			grid_size: GridSize::new(9, 9),
			bomb_count: 10
		}
	}
	pub fn intermediate() -> Self {
		Self {
			grid_size: GridSize::new(16, 16),
			bomb_count: 40
		}
	}
	pub fn expert() -> Self {
		Self {
			grid_size: GridSize::new(30, 16),
			bomb_count: 99
		}
	}
}
impl Default for DifficultySettings {
	fn default() -> Self {
		Self::intermediate()
	}
}


/// Contains various parameters used to generate the graphical representation of a minesweeper game
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct UiStyle {
	pub cell_padding: u16,
	pub header_size: u16,
	pub cell_size: u16,
	pub font_size: u16,
	pub margin: u16,
}
impl Default for UiStyle {
	fn default() -> Self {
		Self {
			cell_padding: 2,
			header_size: 100,
			font_size: 25,
			cell_size: 30,
			margin: 20,
		}
	}
}

/// This structure contain the three previous structures to be easily accessed as a single resource
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameConfig {
	pub difficulty: DifficultySettings,
	pub ui_style: UiStyle,
	pub theme: ColorTheme,
}
