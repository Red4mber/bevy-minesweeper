use bevy::prelude::{Entity, Resource};
use bevy::utils::HashMap;

use crate::components::Coordinates;

mod grid;
pub use grid::*;


/// Hashmap that maps grid coordinates with the "Cover" entity
/// So we can easily uncover them in the `uncover_event_handler`
#[derive(Resource)]
pub struct CoveredCells(pub HashMap<Coordinates, Entity>);
impl Default for CoveredCells {
	fn default() -> Self {
		CoveredCells(HashMap::new())
	}
}

