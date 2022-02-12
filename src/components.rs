use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Player {
  pub move_speed: f32
}

#[derive(Component)]
pub struct Enemy;

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

#[derive(Component)]
pub struct Coin;

pub enum GemValue {
  Low,
  Medium,
  High
}

#[derive(Component)]
pub struct Gem {
  pub value: GemValue
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);