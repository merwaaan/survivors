use bevy::{prelude::*, asset::AssetPath};
use crate::components::*;
use std::{collections::HashMap, path::Path};

// Player

#[derive(Bundle)]
pub struct PlayerBundle {
  player: Player,
  weapons: Weapons,
  health: Health,

  #[bundle]
  sprite: SpriteBundle
}

impl PlayerBundle {
  pub fn new() -> Self {
    let mut weapon_timers: HashMap<WeaponType, Timer> = HashMap::new();
    weapon_timers.insert(WeaponType::Wand, Timer::from_seconds(1.0, true));

    Self {
      player: Player {
        move_speed: 1.0
      },
      health: Health(100),
      weapons: Weapons {
        timers: weapon_timers
      },
      sprite: SpriteBundle {
        sprite: Sprite {
          color: Color::PINK,
          custom_size: Some(Vec2::new(50.0, 50.0)),
          ..Default::default()
        },
        ..Default::default()
      }
    }
  }
}

// Enemy

#[derive(Bundle)]
pub struct EnemyBundle {
  enemy: Enemy,
  health: Health,

  #[bundle]
  sprite: SpriteBundle
}

impl EnemyBundle {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      enemy: Enemy {},
      health: Health(10),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: Color::RED,
          custom_size: Some(Vec2::new(50.0, 50.0)),
          ..Default::default()
        },
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
      }
    }
  }
}

// Projectile

#[derive(Bundle)]
pub struct WandProjectileBundle {

  projectile: Projectile,

  #[bundle]
  sprite: SpriteBundle
}

impl WandProjectileBundle {
  pub fn new(x: f32, y: f32, direction: Vec2) -> Self {
    Self {
      projectile: Projectile { velocity: direction },
      sprite: SpriteBundle {
        sprite: Sprite {
          color: Color::BLUE,
          custom_size: Some(Vec2::new(10.0, 10.0)),
          ..Default::default()
        },
        transform: Transform::from_xyz(x, y, 1.0),
        ..Default::default()
      }
    }
  }
}

// Coin
// TODO merge with gem as Loot?

#[derive(Bundle)]
pub struct CoinBundle {
  coin: Coin,
  animation_timer: AnimationTimer,

  #[bundle]
  sprite: SpriteSheetBundle
}

fn create_sprite_sheet<'a>(
  path: impl Into<AssetPath<'a>>,
  asset_server: &Res<AssetServer>,
  texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
  rows: usize,
  cols: usize
) -> Handle<TextureAtlas> {
  let texture = asset_server.load(path);
  let atlas = TextureAtlas::from_grid(texture, Vec2::new(16.0, 16.0), cols, rows);
  texture_atlases.add(atlas)
}

impl CoinBundle {
  pub fn new(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    x: f32,
    y: f32
  ) -> Self {
    Self {
      coin: Coin { },
      animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
      sprite: SpriteSheetBundle {
        texture_atlas: create_sprite_sheet("coin.png", asset_server, texture_atlases, 1, 5),
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
      }
    }
  }
}

// Gem

#[derive(Bundle)]
pub struct GemBundle {
  gem: Gem,
  animation_timer: AnimationTimer,

  #[bundle]
  sprite: SpriteSheetBundle
}

impl GemBundle {
  pub fn new(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    value: GemValue,
    x: f32,
    y: f32
  ) -> Self {
    let image = match value {
      GemValue::Low => "gem_blue.png",
      GemValue::Medium => "gem_green.png",
      GemValue::High => "gem_red.png"
    };

    Self {
      gem: Gem { value },
      animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
      sprite: SpriteSheetBundle {
        texture_atlas: create_sprite_sheet(image, asset_server, texture_atlases, 1, 4),
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
      }
    }
  }
}