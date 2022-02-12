mod bundles;
mod components;
mod survivors_plugin;
mod systems;
mod events;

use bevy::prelude::*;
use survivors_plugin::SurvivorPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SurvivorPlugin)
    .run();
}