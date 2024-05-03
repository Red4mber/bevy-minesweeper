use bevy::transform::components::GlobalTransform;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Camera, EventWriter, MouseButton, Query, Res, Window};
use bevy::log::debug;

use crate::{
	config::GameConfig,
	events::{FlagCellEvent, UncoverCellEvent}
};

/// Reads mouse events, calculate the cell clicked and send the appropriate events
pub fn mouse_button_events(
	cfg: Res<GameConfig>,
	windows_q: Query<&Window>,
	camera_q: Query<(&Camera, &GlobalTransform)>,
	mouse_button: Res<ButtonInput<MouseButton>>,
	mut ev_uncover: EventWriter<UncoverCellEvent>,
	mut ev_flag: EventWriter<FlagCellEvent>,
) {
	let window = windows_q.single();
	let (camera, camera_transform) = camera_q.single();

	// For every mouse button pressed since last frame
	for button in mouse_button.get_just_pressed() {
		// Get the position of the mouse in the 2D World
		if let Some(world_position) = window.cursor_position()
		                                    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
			// Get the mouse coordinates in the grid
			let mut grid_coord = (world_position - Vec2::splat(cfg.ui_style.margin as f32)) / Vec2::splat( cfg.ui_style.cell_size as f32);

			// If the position is in the grid bounds
			if grid_coord.cmpge(Vec2::ZERO).all() &&
				grid_coord.cmplt(cfg.difficulty.grid_size.into()).all() {
				grid_coord = grid_coord.trunc(); // Truncate to avoid rounding errors

				match button {
					MouseButton::Left => {
						// debug!("Left clicked on cell {} - {}", grid_coord.x, grid_coord.y);
						ev_uncover.send(UncoverCellEvent(grid_coord.into()));
					},
					MouseButton::Right => {
						// debug!("Right clicked on cell {} - {}", grid_coord.x, grid_coord.y);
						ev_flag.send(FlagCellEvent(grid_coord.into()));
					},
					_ => {}
				}
			}
		}
	}
}