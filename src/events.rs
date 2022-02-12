use bevy::prelude::*;

pub struct HitEvent {
  pub target: Entity,
  pub damage: i32
}

pub struct KillEvent {
  pub target: Entity
}
