use crate::{load_level, setupcamera, MyLargeGizmos, Obstacles, ObstaclesRect};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub vel_x: f32,
    pub vel_y: f32,
    pub jumping: bool,
    pub size: f32,
    pub map: i32,
}

#[derive(Component)]
pub struct Segment {
    pub size: f32,
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Player
    let player_size: f32 = 10.;
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.5))),
        Transform::from_translation(Vec3::new(
            0. as f32,
            200. as f32,
            10.0,
        ))
        .with_scale(Vec2::splat(player_size).extend(1.)),
        Player {
            vel_x: 0.0,
            vel_y: 0.0,
            jumping: false,
            size: player_size,
            map: 1,
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

}


pub fn player_controls(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Player>) {
    let jump_power = 3.0;
    let speed = 2.0;
    for mut player in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.vel_x = -speed;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.vel_x = speed;
        } else {
            player.vel_x = 0.0;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) && player.jumping == false {
            player.vel_y = jump_power;
            player.jumping = true;
        }
    }
}

pub fn player_movements(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    obstacle_query: Query<&mut Transform, (With<Obstacles>, Without<Player>)>,
) {
    let mut player_move_off_screen = false;
    for (mut transform, mut player) in player_query.iter_mut() {
        transform.translation.x += player.vel_x;
        if transform.translation.y >= -91.0 {
            player.vel_y -= 0.1;
        } else {
            transform.translation.y = -91.0;
            player.jumping = false;
        }
        transform.translation.y += player.vel_y;

        if transform.translation.x > 220. {
            transform.translation.y = 0.0;
            transform.translation.x = -220.0;
            player_move_off_screen = true;
            player.map += 1;
        }
        if transform.translation.x < -220. {
            transform.translation.y = 0.0;
            transform.translation.x = 220.0;
            player_move_off_screen = true;
            player.map -= 1;
        }
    }

    if player_move_off_screen {
        load_level(obstacle_query);
    }
}


pub fn collision_check_player(
    mut query_player: Query<(&mut Transform, &mut Player)>,
    query_obstacles: Query<(&Transform, &Obstacles), (With<Obstacles>, Without<Player>)>,
) {
    for (mut player_transform, mut player) in query_player.iter_mut() {
        for (obstacle_transform, obstacle) in query_obstacles.iter() {
            let player_position = player_transform.translation;
            let obstacle_position = obstacle_transform.translation;

            let distance = player_position.distance(obstacle_position);
            let player_radius = player.size / 2.;
            let obstacle_radius = obstacle.size / 2.; // Access the size here
            if distance < player_radius + obstacle_radius {
                let shift_vector = player_position - obstacle_position;
                let shift_distance = player_radius + obstacle_radius - distance;
                let shift = shift_vector.normalize() * shift_distance;

                player_transform.translation.x += shift.x;
                player_transform.translation.y += shift.y;
                player.vel_y = 0.0;
                player.jumping = false;
            }
        }
    }
}


// Utility function to constrain a value between a minimum and maximum
fn constrain(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

// Function to calculate distance between two points
fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// Rectangle collision check
pub fn collision_check_player_rect(
    mut query_object: Query<(&mut Transform, &mut Player)>,
    query_obstacles: Query<(&Transform, &ObstaclesRect), (With<ObstaclesRect>, Without<Player>)>,
) {
    for (mut object_transform, mut object) in query_object.iter_mut() {
        for (obstacle_transform, obstacle) in query_obstacles.iter() {
            let closest_x = constrain(
                object_transform.translation.x,
                obstacle_transform.translation.x - (obstacle.size / 2.0),
                obstacle_transform.translation.x - (obstacle.size / 2.0) + obstacle.size,
            );
            let closest_y = constrain(
                object_transform.translation.y,
                obstacle_transform.translation.y - (obstacle.size / 2.0),
                obstacle_transform.translation.y - (obstacle.size / 2.0) + obstacle.size,
            );

            let d = distance(
                object_transform.translation.x,
                object_transform.translation.y,
                closest_x,
                closest_y,
            );

            if d < (object.size / 2.) {
                let shift_x = object_transform.translation.x - closest_x;
                let shift_y = object_transform.translation.y - closest_y;
                let shift_magnitude = (object.size / 2.) - d;

                let shift_vector = Vec3::new(shift_x, shift_y, 0.0).normalize() * shift_magnitude;

                object_transform.translation.x += shift_vector.x;
                object_transform.translation.y += shift_vector.y;
                object.vel_y = 0.0;
                object.jumping = false;
            }
        }
    }
}
