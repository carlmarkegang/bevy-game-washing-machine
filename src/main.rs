use bevy::prelude::*;
use rand::Rng;
mod setupcamera;
mod setupplayer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Startup,
            (
                setupcamera::setup_camera,
                setup_main,
                setupplayer::setup_player,
            ),
        )
        .add_systems(
            Update,
            (
                setupcamera::fit_canvas, 
                setupplayer::rotate_circle
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                setupplayer::collision_check_player,
                setupplayer::collision_check_player_rect,
                setupplayer::player_controls,
                setupplayer::player_movements,
                backgroundpixles_movement,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Backgroundpixles;

#[derive(Component)]
struct Obstacles {
    size: f32,
}

#[derive(Component)]
struct ObstaclesRect {
    size: f32,
}


fn setup_main(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("pixel/background.png")),
        Transform::from_xyz(0., 0., 1.),
        setupcamera::PIXEL_PERFECT_LAYERS,
        Background,
    ));

    // Background pixels
    for _i in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(
                generate_random_int(-90..90) as f32,
                generate_random_int(-90..90) as f32,
                2.,
            )
            .with_scale(Vec3::new(1.0, 1.0, 2.0)),
            Backgroundpixles,
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }

    // Obstacles
    let obstacle_size = 20.0;
    let mut pos_x = -200.0;
    for _i in 0..30 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
            Transform::from_xyz(
                pos_x,
                -100.0,
                4.,
            )
            .with_scale(Vec3::splat(obstacle_size)),
            ObstaclesRect { size: 20.0 },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
        pos_x += 20.0;
    }

    // Obstacles Rect
    pos_x = -200.0;
    let obstacle_size = Vec3::new(10.0, 10.0, 1.0);
    for _i in 0..10 {
       
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
            Transform::from_xyz(
                pos_x,
                -60.0,
                4.,
            )
            .with_scale(obstacle_size),
            ObstaclesRect { size: 10.0 },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
        pos_x += 10.0;
    }
}

fn backgroundpixles_movement(mut transforms: Query<&mut Transform, With<Backgroundpixles>>) {
    for mut transform in &mut transforms {
        if generate_random_int(0..20) == 0 {
            transform.translation.x = generate_random_int(-200..200) as f32;
        }
    }
}

fn generate_random_int(maxmin: std::ops::Range<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let generated_float = rng.gen_range(maxmin);
    generated_float
}
