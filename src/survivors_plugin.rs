use bevy::prelude::*;

use crate::bundles::*;
use crate::events::*;
use crate::systems::*;

pub struct SurvivorPlugin;

impl Plugin for SurvivorPlugin {
  fn build(&self, app: &mut App) {
    app
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
      .add_system(loot_drop_system);
  }
}

fn setup(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // TODO make bundles for player, enemy

  commands.spawn_bundle(PlayerBundle::new());

  commands.spawn_bundle(EnemyBundle::new(100.0, 100.0));
  commands.spawn_bundle(EnemyBundle::new(-100.0, -50.0));

  /*command.spawn_bundle(SpriteBundle {
    sprite: Sprite {
      color: Color::RED,
      custom_size: Some(Vec2::new(50.0, 50.0)),
      ..Default::default()
    },
    transform: Transform::from_xyz(x, y, 0.0),
    ..Default::default()
  })
  .insert(ExperienceBar)*/
}
