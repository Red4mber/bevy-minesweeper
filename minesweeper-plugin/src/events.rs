
use bevy::prelude::*;
use bevy::log::debug;

use crate::{
	AppState, Coordinates,
	config::GameConfig,
	resources::{CoveredCells, NEIGHBORS},
	components::{Bomb, EmptyCell, Flag, Uncover},
};
use crate::components::GridContainer;
use crate::config::DifficultySettings;


#[derive(Debug, Copy, Clone, Event)]
pub struct UncoverCellEvent(pub Coordinates);

#[derive(Debug, Copy, Clone, Event)]
pub struct FlagCellEvent(pub Coordinates);

#[derive(Debug, Copy, Clone, Event)]
pub struct ExplosionEvent;

#[derive(Debug, Copy, Clone, Event)]
pub struct RestartEvent;

// #[derive(Debug, Copy, Clone, Event)]
// pub struct VictoryEvent;

#[derive(Event)]
pub struct ChangeDifficultyEvent(pub DifficultySettings);

pub fn update_difficulty(
	mut ev_difficulty: EventReader<ChangeDifficultyEvent>,
	mut ev_restart: EventWriter<RestartEvent>,
	mut cfg: ResMut<GameConfig>
) {
	for ev in ev_difficulty.read() {
		cfg.difficulty = ev.0;
	}
	ev_restart.send(RestartEvent);
}



/// Despawns the grid so we can later generate a new one
pub fn despawn_grid(
	mut cmd: Commands,
		q_grid: Query<Entity, With<GridContainer>>,
) {
	info!("Despawning the grid to restart the game...");
	if let Ok(entity) = q_grid.get_single() {
		cmd.entity(entity).despawn_recursive()
	}
}

/// Handles UncoverCellEvents
///
/// We cannot simply access the cover's Parent so we instead add a `Uncover` marker it
/// so we can later query all marked Covers and their parents
/// hence allowing us to check if the cell contains a bomb
pub fn uncover_event_handler(
		cfg: Res<GameConfig>,
	mut cmd: Commands,
	mut covered_cells: ResMut<CoveredCells>,
	mut next_state: ResMut<NextState<AppState>>,
	// mut ev_victory: EventWriter<VictoryEvent>,
	mut ev_uncover: EventReader<UncoverCellEvent>,
) {
	for ev in ev_uncover.read() {
		let cover = covered_cells.0.remove(&ev.0);
		if covered_cells.0.iter().count() == cfg.difficulty.bomb_count as usize {
			next_state.set(AppState::Victory);
		}
		match cover {
			None => {
				// debug!("The Cell {} is already uncovered !", ev.0); // Big spammer
			},
			Some(entity) => {
				// debug!("Uncovering cell {}", ev.0); // Big spammer
				cmd.entity(entity).insert(Uncover);
			},
		}
	}
}

/// Queries the children all cells with a `Uncover` marker, then delete
pub fn uncover_cell(
	mut cmd: Commands,
	mut ev_explosion: EventWriter<ExplosionEvent>,
	mut ev_uncover: EventWriter<UncoverCellEvent>,
		q_cell: Query<(Entity, &Parent), With<Uncover>>,
		q_parents: Query<(&Coordinates, Option<&Bomb>, Option<&EmptyCell>)>,
) {
	for (entity, parent) in q_cell.iter() {
		cmd.entity(entity).despawn_recursive();
		if let Ok((coord, is_bomb, is_empty)) = q_parents.get(parent.get()) {
			if is_bomb.is_some() {
				ev_explosion.send(ExplosionEvent);
			}
			if is_empty.is_some() {
				// info!("Cell {coord} is empty, propagating event to all adjacent empty cells"); // Big spammer
				// Iterates over all neigbors of the cell and sends an event for each one
				NEIGHBORS.iter()
					.for_each(|neighbor| {
						let neighbor_coords = *coord + *neighbor;
						ev_uncover.send(UncoverCellEvent(neighbor_coords));
					});
			}
		}
	}
}

/// Handles FlagCellEvent, toggling the flags on the suspected trapped cells
///
/// This time i tried an other technique than uncover cells.
/// We query every cell, then check if the coordinates match with the event, is they do match,
/// we then check if there's a flag already present on the cell, If there is a flag, we remove it,
/// If there is no flag on the cell, we add a flag_sprite and a Flag marker
pub fn flag_event_handler(
	mut ev_flags: EventReader<FlagCellEvent>,
	mut cmd: Commands,
		srv: Res<AssetServer>,
		cfg: Res<GameConfig>,
		q_cells: Query<(&Coordinates, Entity, Option<&Flag>)>,
) {
	for ev in ev_flags.read() {
		debug!("Toggling Flag on cell {}", ev.0);
		for (coord, entity, flag) in &q_cells {
			if *coord == ev.0 {
				match flag {
					None => {
						let flag_sprite = srv.load("sprites/flag.png");
						let mut child_id: Flag = Flag { id: entity }; // Placeholder value, because cargo won't compile without
						cmd.entity(entity)
						   .with_children(|parent| {
							   // Spawn a child containing the Flag sprite
							   let id = parent.spawn(SpriteBundle {
								   sprite: Sprite {
									   custom_size: Some(Vec2::splat((cfg.ui_style.cell_size - cfg.ui_style.cell_padding - 2) as f32)),
									   ..default()
								   },
								   transform: Transform::from_xyz(
									   cfg.ui_style.cell_size as f32 / 2.,
									   cfg.ui_style.cell_size as f32 / 2.,
									   3.
								   ),
								   texture: flag_sprite.clone(),
								   ..default()
							   }).insert(bevy::core::Name::new("Flag")).id();
							   // Store the child's ID in a component
							   child_id = Flag { id };
						   }).insert(child_id);
					},
					Some(&flag_id) => {
						// Despawn the flag
						cmd.entity((flag_id).id).despawn();
						// Remove the component containing the flag's id
						cmd.entity(entity).remove::<Flag>();
					}
				}
			}
		}
	}
}

/// Just sets game state to GameOver, everything else is handled by the Game State
pub fn explosion_event_handler(
	ev_explosion: EventReader<ExplosionEvent>,
	mut next_state: ResMut<NextState<AppState>>,
) {
	info!("Tough luck, you just blew up! Try skill next time.");
	next_state.set(AppState::GameOver);
}