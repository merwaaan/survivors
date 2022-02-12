use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::bundles::*;
use crate::events::*;
use crate::systems::*;

pub struct SurvivorPlugin;

impl Plugin for SurvivorPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(ShapePlugin)
      .insert_resource(ClearColor(Color::rgb(0.0, 0.3, 0.0)))
      .add_event::<HitEvent>()
      .add_event::<KillEvent>()
      .add_startup_system(setup)
      .add_system(animation_system)
      .add_system(player_movement_system)
      .add_system(player_attack_system)
      .add_system(enemy_spawn_system)
      .add_system(enemy_movement_system)
      .add_system(enemy_damage_system)
      .add_system(projectile_movement_system)
      .add_system(collision_system)
      .add_system(collision_debug_system)
      .add_system(loot_drop_system);
  }
}

fn setup(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(PlayerBundle::new());

}
