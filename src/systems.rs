use bevy::prelude::*;
use bevy::reflect::List;
use bevy::sprite::collide_aabb::collide;
use bevy::transform;
use crate::bundles::*;
use crate::components::*;
use crate::events::*;
use random_wheel::RandomWheel;

pub fn player_movement_system(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&Player, &mut Transform)>
) {
  let player = player_query.single().0;
  let mut transform = player_query.single_mut().1;

  if keyboard_input.pressed(KeyCode::Left) {
    transform.translation.x -= player.move_speed;
  }
  else if keyboard_input.pressed(KeyCode::Right) {
    transform.translation.x += player.move_speed;
  }

  if keyboard_input.pressed(KeyCode::Up) {
    transform.translation.y += player.move_speed;
  }
  else if keyboard_input.pressed(KeyCode::Down) {
    transform.translation.y -= player.move_speed;
  }
}

pub fn enemy_spawn_system(
  mut commands: Commands,
  enemies_query: Query<&Enemy>/*,
  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>*/
) {
  if enemies_query.iter().count() < 10 {
    let x = 10 - enemies_query.iter().count();

    for i in 0..x {
      commands.spawn_bundle(EnemyBundle::new(rand::random::<f32>() * 1000.0, rand::random::<f32>() * 1000.0));
    }
  }
}

pub fn enemy_movement_system(
  mut enemies_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>
) {
  let player_position = &player_query.single().translation;

  for mut transform in enemies_query.iter_mut() {
    let enemy_position = &mut transform.translation;

    let to_player = (*player_position - *enemy_position).normalize();

    *enemy_position += to_player * 0.2;
  }
}

pub fn enemy_damage_system(
  mut commands: Commands,
  mut read_hit_events: EventReader<HitEvent>,
  mut send_kill_event: EventWriter<KillEvent>,
  mut enemies_query: Query<(&Enemy, &mut Health)>
) {
  for event in read_hit_events.iter() {
    let health = enemies_query.get_component_mut::<Health>(event.target);

    match health {
      Ok(mut h) => {
        h.0 -= event.damage;

        if h.0 <= 0 {
          commands.entity(event.target).despawn();
          send_kill_event.send(KillEvent { target: event.target });
        }
      },
      _ => println!("cannot get enemy health")
    }
  }
}

pub fn player_attack_system(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<&mut Weapons>,
  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
  enemies_query: Query<&Transform, (With<Enemy>, Without<Player>)>
) {
  let player_position = &player_query.single().translation;

  for mut weapons in query.iter_mut() {
    for (weapon_type, weapon_timer) in weapons.timers.iter_mut() {
      if weapon_timer.tick(time.delta()).just_finished() {
        match weapon_type {
          WeaponType::Wand => {

            // Look for the closest enemy

            let mut closest: Option<&Transform> = None;
            let mut closest_distance = 999999.0; // TODO inf

            for transform in enemies_query.iter() {
              let distance = (transform.translation.x - player_position.x).abs() + (transform.translation.y - player_position.y).abs();
              if distance < closest_distance {
                closest = Some(&transform);
                closest_distance = distance;
              }
            }

            let direction = match closest {
              Some(enemy) => Vec2::new(enemy.translation.x, enemy.translation.y) - Vec2::new(player_position.x, player_position.y),
              None => Vec2::new(rand::random::<f32>(), rand::random::<f32>())
            };

            commands.spawn_bundle(WandProjectileBundle::new(player_position.x, player_position.y, direction.normalize()));
          }
        }
      }
    }
  }
}

pub fn projectile_movement_system( // TODO split in projectile_collision_system
  mut commands: Commands,
  mut send_hit_event: EventWriter<HitEvent>,
  mut projectile_query: Query<(Entity, &Projectile, &mut Transform, &Sprite)>,
  enemies_query: Query<(Entity, &Transform, &Sprite), (With<Enemy>, Without<Projectile>)>
) {
  // Move

  for (_projectile, projectile_, mut transform, _) in projectile_query.iter_mut() {
    transform.translation.x += projectile_.velocity.x;
    transform.translation.y += projectile_.velocity.y;
  }

  // Collides?

  for (projectile, _projectile_, projectile_transform, projectile_sprite) in projectile_query.iter() {
    for (enemy, enemy_tranform, enemy_sprite) in enemies_query.iter() {
      match collide(
        projectile_transform.translation,
        projectile_sprite.custom_size.unwrap(),
        enemy_tranform.translation,
        enemy_sprite.custom_size.unwrap()
      ) {
        Some(_) => {
          send_hit_event.send(HitEvent { target: enemy, damage: 5 });
          commands.entity(projectile).despawn();
        }
        _ => {}
      }
    }
  }
}

pub fn loot_drop_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut kill_events: EventReader<KillEvent>,
  mut enemies_query: Query<(&Enemy, &mut Transform)>
) {
  for event in kill_events.iter() {
    let transform = enemies_query.get_component_mut::<Transform>(event.target);

    match transform {
      Ok(t) => {

        enum DropType {
          Nothing,
          Gem,
          Coin,
        }

        let mut type_wheel: RandomWheel::<DropType> = RandomWheel::new();
        type_wheel.push(10.0, DropType::Nothing);
        type_wheel.push(10.0, DropType::Gem);
        type_wheel.push(5.0, DropType::Coin);
        let loot_type = type_wheel.pop().unwrap().1;

        match loot_type {
          DropType::Nothing => {}

          DropType::Gem => {
            let mut wheel: RandomWheel::<GemValue> = RandomWheel::new();
            wheel.push(10.0, GemValue::Low);
            wheel.push(1.0, GemValue::Medium);
            wheel.push(0.1, GemValue::High);
            let value = wheel.pop().unwrap().1;

            commands.spawn_bundle(LootBundle::new(&asset_server, &mut texture_atlases, LootType::Gem(value), t.translation.x, t.translation.y));
          }

          DropType::Coin => {
            commands.spawn_bundle(LootBundle::new(&asset_server, &mut texture_atlases, LootType::Coin, t.translation.x, t.translation.y));
          }
        }
      },
      _ => panic!("cannot get enemy transform")
    }
  }
}


pub fn animation_system(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut animated_query: Query<(
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>
) {
  for (mut timer, mut sprite, atlas_handle) in animated_query.iter_mut() {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
      let atlas = texture_atlases.get(atlas_handle).unwrap();
      sprite.index = (sprite.index + 1) % atlas.textures.len();
    }
  }
}
