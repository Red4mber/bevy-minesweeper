use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use crate::config::GameConfig;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum SettingsState {
	Main,
	Colors,
	Difficulty,
	#[default]
	Disabled,
}


pub fn spawn_settings_ui(
	_cmd: Commands,
	_cfg: Res<GameConfig>,
	_srv: Res<AssetServer>,
) {

}


pub fn despawn_settings_ui(
	_cmd: Commands,
	_cfg: Res<GameConfig>,
	_srv: Res<AssetServer>,
) {

}