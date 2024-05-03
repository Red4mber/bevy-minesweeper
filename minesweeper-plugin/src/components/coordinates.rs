
use core::{fmt, ops};
use bevy::prelude::*;


// The Coordinates component is a custom type describing a cell's coordinate in the game's grid


#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Coordinates {
	pub x: u16,
	pub y: u16,
}
impl Coordinates {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
}
impl From<Vec2> for Coordinates {
	fn from(item: Vec2) -> Self {
		Coordinates { x: item.x as u16, y: item.y as u16 }
	}
}
impl Into<Vec2> for Coordinates {
	fn into(self) -> Vec2 {
		Vec2::new(self.x.into(), self.y.into())
	}
}
impl fmt::Display for Coordinates {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
impl ops::Add for Coordinates {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl ops::Add<(i8, i8)> for Coordinates {
	type Output = Self;
	fn add(self, (x, y): (i8, i8)) -> Self::Output {
		let x = ((self.x as i16) + x as i16) as u16;
		let y = ((self.y as i16) + y as i16) as u16;
		Self { x, y }
	}
}
impl ops::Sub for Coordinates {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x.saturating_sub(rhs.x),
			y: self.y.saturating_sub(rhs.y),
		}
	}
}