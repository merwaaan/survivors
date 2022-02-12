use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Player {
  pub move_speed: f32
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct HitBox(pub f32); // The hitbox is a square, the value is half its size

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Projectile {
  pub velocity: Vec2
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum WeaponType {
  Wand
}

#[derive(Component)]
pub struct Weapons {
  pub timers: HashMap<WeaponType, Timer>
}

pub enum GemValue {
  Low,
  Medium,
  High
}

pub enum LootType
{
  Coin,
  Gem(GemValue)
}

#[derive(Component)]
pub struct Loot(pub LootType);

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
