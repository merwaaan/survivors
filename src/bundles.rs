use bevy::{prelude::*, asset::AssetPath};
use crate::components::*;
use std::{collections::HashMap};

// Player

#[derive(Bundle)]
pub struct PlayerBundle {
  player: Player,
  weapons: Weapons,
  health: Health,
  hitbox: HitBox,

  #[bundle]
  sprite: SpriteBundle
}

impl PlayerBundle {
  pub fn new() -> Self {
    let size = 50.0;

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
      hitbox: HitBox(size * 0.5),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: Color::PINK,
          custom_size: Some(Vec2::new(size, size)),
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
  hitbox: HitBox,

  #[bundle]
  sprite: SpriteBundle
}

impl EnemyBundle {
  pub fn new(x: f32, y: f32) -> Self {
    let size = 50.0;

    Self {
      enemy: Enemy {},
      health: Health(10),
      hitbox: HitBox(size * 0.5),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: Color::RED,
          custom_size: Some(Vec2::new(size, size)),
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
  hitbox: HitBox,

  #[bundle]
  sprite: SpriteBundle
}

impl WandProjectileBundle {
  pub fn new(x: f32, y: f32, direction: Vec2) -> Self {
    let size = 10.0;

    Self {
      projectile: Projectile { velocity: direction },
      hitbox: HitBox(size * 0.5),
      sprite: SpriteBundle {
        sprite: Sprite {
          color: Color::BLUE,
          custom_size: Some(Vec2::new(size, size)),
          ..Default::default()
        },
        transform: Transform::from_xyz(x, y, 1.0),
        ..Default::default()
      }
    }
  }
}

// Loot

#[derive(Bundle)]
pub struct LootBundle {
  loot: Loot,
  hitbox: HitBox,
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

impl LootBundle {
  pub fn new(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    loot_type: LootType,
    x: f32,
    y: f32
  ) -> Self {
    let sprite_sheet = match loot_type {
      LootType::Coin => create_sprite_sheet("coin.png", asset_server, texture_atlases, 1, 5),
      LootType::Gem(GemValue::Low) => create_sprite_sheet("gem_blue.png", asset_server, texture_atlases, 1, 4),
      LootType::Gem(GemValue::Medium) => create_sprite_sheet("gem_green.png", asset_server, texture_atlases, 1, 4),
      LootType::Gem(GemValue::High) => create_sprite_sheet("gem_red.png", asset_server, texture_atlases, 1, 4)
    };

    Self {
      loot: Loot(loot_type),
      hitbox: HitBox(5.0),
      animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
      sprite: SpriteSheetBundle {
        texture_atlas: sprite_sheet,
        transform: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
      }
    }
  }
}

// Hit point

#[derive(Bundle)]
pub struct HitPointBundle {
  hit_point: HitPoint,
  animated_text: AnimatedText,

  #[bundle]
  text: Text2dBundle
}

impl HitPointBundle {
  pub fn new(
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
    value: i32
  ) -> Self {
    let font = asset_server.load("Minimal5x7.ttf");

    Self {
      hit_point: HitPoint(value),
      animated_text: AnimatedText {
        target_scale: 2.0,
        speed: 0.005,
      },
      text: Text2dBundle {
        text: Text::with_section(
          value.to_string(),
          TextStyle {
            font,
            font_size: 30.0,
            color: Color::WHITE,
        },
        TextAlignment {
          horizontal: HorizontalAlign::Center,
          vertical: VerticalAlign::Center
        }),
        transform: Transform::from_xyz(x, y, 1.0),
        ..Default::default()
      }
    }
  }
}
