use bevy::log::debug;
use std::time::Instant;
use bevy::prelude::{Res, ResMut, Resource};
use bevy::time::Stopwatch;

#[derive(Resource)]
pub struct GameTime {
	pub stopwatch: Stopwatch,
	last_tick: Instant,
}
impl Default for GameTime {
	fn default() -> Self {
		Self {
			stopwatch: Stopwatch::new(),
			last_tick: Instant::now()
		}
	}
}
impl GameTime {
	pub fn pause(&mut self) {
		self.stopwatch.pause()
	}
	pub fn unpause(&mut self) {
		self.stopwatch.unpause()
	}
	// pub fn toggle_pause(&mut self) {
	// 	if self.stopwatch.paused() { self.stopwatch.unpause() } else { self.stopwatch.pause() }
	// }
	pub fn restart(&mut self) {
		self.stopwatch.reset()
	}
	pub fn update(&mut self) {
		let now = Instant::now();
		let delta = now - self.last_tick;
		self.stopwatch.tick(delta);
		self.last_tick = now;
	}
	pub fn to_string(&self) -> String {
		let secs = self.stopwatch.elapsed_secs();
		if secs > 60.0 { format!("{} mins {} secs", (secs / 60.).trunc(), (secs % 60.).trunc()) }
		else { format!("{} secs", secs.trunc()) }
	}
}

pub fn update_timer( mut timer: ResMut<GameTime> ) {
	timer.update()
}
pub fn pause_timer( mut time: ResMut<GameTime> ) {
	debug!("Pausing timer");
	time.pause();
}
pub fn unpause_timer( mut time: ResMut<GameTime> ) {
	debug!("Unpausing timer");
	time.unpause();
}
pub fn restart_timer( mut time: ResMut<GameTime> ) {
	debug!("Restarting timer");
	time.restart()
}
