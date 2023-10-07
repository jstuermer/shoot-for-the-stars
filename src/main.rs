use bevy::{prelude::*, window::PrimaryWindow};
use rand::{seq::SliceRandom, Rng};

pub const PLAYER_SIZE: f32 = 64.0; // this is the size of the player sprite
pub const ENEMY_SIZE: f32 = 64.0; // this is the size of the enemy sprite
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_TIMESTEP: f32 = 1.0;
pub const COLLISION_REBOUND_STRENGTH: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies))
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement.after(player_movement),
                enemy_movement,
                confine_enemy_movement.after(enemy_movement),
                enemy_hit_player,
            ),
        )
        .add_systems(
            FixedUpdate,
            enemy_redirection.before(confine_enemy_movement),
        )
        .insert_resource(FixedTime::new_from_secs(ENEMY_TIMESTEP))
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    direction: Vec3,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;
    let x_min: f32 = half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x_position: f32 = rng.gen_range(x_min..=x_max);
        let y_position: f32 = rng.gen_range(y_min..=y_max);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_position, y_position, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::ZERO,
            },
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn enemy_redirection(mut enemy_query: Query<&mut Enemy>) {
    let sample_directions: [f32; 3] = [-1.0, 0.0, 1.0];
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for mut enemy in &mut enemy_query {
        let mut direction = Vec3::ZERO;
        let x_random: &f32 = sample_directions
            .choose(&mut rng)
            .expect("Random x direction should have been generated.");
        let y_random: &f32 = sample_directions
            .choose(&mut rng)
            .expect("Random y direction should have been generated.");
        direction += Vec3::new(*x_random, *y_random, 0.0);
        enemy.direction = direction.normalize_or_zero();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in &mut enemy_query {
        enemy_transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;

    let x_min: f32 = half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (mut enemy_transform, mut enemy) in &mut enemy_query {
        let mut changed_direction: bool = false;

        if enemy_transform.translation.x < x_min {
            enemy_transform.translation.x = x_min;
            enemy.direction.x = -enemy.direction.x;
            changed_direction = true;
        } else if enemy_transform.translation.x > x_max {
            enemy_transform.translation.x = x_max;
            enemy.direction.x = -enemy.direction.x;
            changed_direction = true;
        }
        if enemy_transform.translation.y < y_min {
            enemy_transform.translation.y = y_min;
            enemy.direction.y = -enemy.direction.y;
            changed_direction = true;
        } else if enemy_transform.translation.y > y_max {
            enemy_transform.translation.y = y_max;
            enemy.direction.y = -enemy.direction.y;
            changed_direction = true;
        }

        if changed_direction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/pluck_001.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }

        direction = direction.normalize_or_zero();

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();
        let half_player_size: f32 = PLAYER_SIZE / 2.0;

        let x_min: f32 = half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        if player_transform.translation.x < x_min {
            player_transform.translation.x = x_min;
        }
        if player_transform.translation.x > x_max {
            player_transform.translation.x = x_max;
        }
        if player_transform.translation.y < y_min {
            player_transform.translation.y = y_min;
        }
        if player_transform.translation.y > y_max {
            player_transform.translation.y = y_max;
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((_player_entity, mut player_transform)) = player_query.get_single_mut() {
        let collision_distance = (PLAYER_SIZE + ENEMY_SIZE) / 2.0;
        for mut enemy_transform in &mut enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance > collision_distance {
                continue;
            }

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/explosionCrunch_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });

            let mut relative_vector = player_transform.translation - enemy_transform.translation;
            relative_vector.z = 0.0; // z-ordering must be neglected
            relative_vector = relative_vector.normalize_or_zero();
            enemy_transform.translation -= COLLISION_REBOUND_STRENGTH * relative_vector;
            player_transform.translation += COLLISION_REBOUND_STRENGTH * relative_vector;
        }
    }
}
