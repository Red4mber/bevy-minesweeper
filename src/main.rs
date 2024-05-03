use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::EnabledButtons;
use minesweeper_plugin::MinesweeperPlugin;


fn main() {
    let mut app = App::new();
     app.add_plugins(DefaultPlugins.set(
         WindowPlugin {
             primary_window: Some(Window {
                 resizable: false,
                 title: "Minesweeper".into(),
                 name:  Some("Minesweeper".into()),
                 resolution: (1., 1.).into(),
                 visible: false,
                 enabled_buttons: EnabledButtons { maximize: false, ..default() },
                 ..default()
             }),
             ..default()
         }
     ).set(LogPlugin {
         level: bevy::log::Level::DEBUG,
         filter: "warn,wgpu_core=warn,wgpu_hal=warn,minesweeper=debug".into(),
         ..default()
     }));
    app.add_plugins(MinesweeperPlugin);
    app.run();
}
