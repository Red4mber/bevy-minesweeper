
// This module contains the implementation of a Minesweeper Grid in native rust
// Uses no befy functionnalities, allowing it to be reused for other minesweeper-related projects

use std::ops::{Deref, DerefMut};
use bevy::log::debug;
use rand::Rng;

use crate::components::Coordinates;

/// Slice containing the position of all neighbors of a cell
pub const NEIGHBORS: [(i8, i8); 8] = [
	(-1,  1), (0,  1), (1,  1),
	(-1,  0),          (1,  0),
	(-1, -1), (0, -1), (1, -1),
];


/// A enum that describes a single cell in the gane grid
///
/// It can either be a Bomb, an empty cell or a Bomb neighbor (in which case it stores the number of neigboring bombs)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
	Bomb,
	BombNeighbor(u8),
	Empty,
}

impl Cell {
	/// Returns true if the cell is a bomb
	pub fn is_bomb(&self) -> bool {
		matches!(self, Self::Bomb)
	}
	/// Returns an ASCII representation of the cell
	pub fn console_output(&self) -> String {
		format!("{}", match self {
			Cell::Empty => " ".to_string(),
			Cell::Bomb => "*".to_string(),
			Cell::BombNeighbor(v) => v.to_string(),
		})
	}
}


/// A structure that describes a minesweeper grid of cells
///
/// The grid is stored as a 2D Vector of Cells
/// Every field is private, but there is getters for everything needed for the game
pub struct Grid {
	bomb_count: u16,
	height: u16,
	width: u16,
	grid: Vec<Vec<Cell>>,
}
/// Allows easy access to the contained grid by dereferencing the structure
impl Deref for Grid {
	type Target = Vec<Vec<Cell>>;
	fn deref(&self) -> &Self::Target { &self.grid }
}
impl DerefMut for Grid {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.grid }
}

impl Grid {
	/// Create an empty grid with no bomb
	pub fn empty(width: u16, height: u16) -> Self {
		let map = (0..height)
			.into_iter()
			.map(|_| (0..width).into_iter().map(|_| Cell::Empty).collect())
			.collect();
		Self {
			bomb_count: 0,
			height,
			width,
			grid: map,
		}
	}

	/// Spawns an abritrary number of bombs in the grid
	pub fn set_bombs(&mut self, count: u16) {
		let mut rng = rand::thread_rng();
		let mut remaining = count;
		while remaining > 0 {
			let (x, y) = (
				rng.gen_range(0..self.width) as usize,
				rng.gen_range(0..self.height) as usize,
			);
			if let Cell::Empty = self[y][x] {
				self.grid[y][x] = Cell::Bomb;
				remaining -= 1;
			}
		}
		self.update_neighbors();
		self.bomb_count = count;
	}

	/// Updates every cell in the grid to contain the count of bomb among the cell's neighbors
	fn update_neighbors(&mut self) {
		for y in 0..self.height {
			for x in 0..self.width {
				let coords = Coordinates { x, y };
				if self.is_bomb_at(coords) { continue }
				let bombs = self.count_bomb_neighbors(coords);
				if bombs > 0 {
					let cell = &mut self[y as usize][x as usize];
					*cell = Cell::BombNeighbor(bombs)
				}
			}
		}
	}

	/// Returns true is the cell at coordinates contains a bomb
	pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
		if coordinates.x >= self.width || coordinates.y >= self.height { return false; };
		self.grid[coordinates.y as usize][coordinates.x as usize].is_bomb()
	}

	/// Returns an iterator of the neighbors of the cell at the coordinates specified
	pub fn get_neighbors(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
		NEIGHBORS.iter()
			.copied()
			.map(move |neighbor_coordinates| coordinates + neighbor_coordinates)
	}

	/// Returns the number of neighbors of a cell at specified coordinates that contains a bomb
	pub fn count_bomb_neighbors(&self, coordinates: Coordinates) -> u8 {
		if self.is_bomb_at(coordinates) { return 0 }
		self.get_neighbors(coordinates)
			.filter(|&coord| self.is_bomb_at(coord))
			.count() as u8
	}

	// Some getters that might or might not be useful
	pub fn width(&self) -> u16 { self.width }
	pub fn height(&self) -> u16 { self.height }
	pub fn bomb_count(&self) -> u16 { self.bomb_count }

	/// Returns a String containing the grids ASCII representation, for logging/debugging purposes
	pub fn console_output(&self) -> String {
		let mut buffer = format!(
			"Map ({}, {}) with {} bombs:\n",
			self.width, self.height, self.bomb_count
		);
		let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
		buffer = format!("{}{}\n", buffer, line);
		for line in self.iter().rev() {
			buffer = format!("{}|", buffer);
			for cell in line.iter() {
				buffer = format!("{}{}", buffer, cell.console_output());
			}
			buffer = format!("{}|\n", buffer);
		}
		format!("{}{}", buffer, line)
	}
}
