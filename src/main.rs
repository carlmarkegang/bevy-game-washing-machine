use bevy::prelude::*;
use rand::Rng;
mod setupcamera;
mod setupplayer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_gizmo_group::<MyLargeGizmos>()
        .add_systems(
            Startup,
            (
                setupcamera::setup_camera,
                setup_main,
                setupplayer::setup_player,
                set_line_width,
            ),
        )
        .add_systems(
            Update,
            (
                setupcamera::fit_canvas, 
                //setupplayer::rotate_circle
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
                change_background,
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

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyLargeGizmos {}

#[derive(Component)]
struct MapText;

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
    for _i in 0..80 {
        let obstacle_size = generate_random_int(30..50) as f32;
        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
            Transform::from_xyz(
                generate_random_int(-200..200) as f32,
                generate_random_int(-100..-80) as f32,
                4.,
            )
            .with_scale(Vec3::splat(obstacle_size)),
            Obstacles {
                size: obstacle_size,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }

    // Obstacles Rect
    for _i in 0..5 {
        let obstacle_size = Vec3::new(20.0, 20.0, 1.0);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
            Transform::from_xyz(
                generate_random_int(-200..200) as f32,
                generate_random_int(-40..50) as f32,
                4.,
            )
            .with_scale(obstacle_size),
            ObstaclesRect { size: 20.0 },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }

    commands.spawn((
        Text::new("Map"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
        MapText,
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));
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

fn load_level(
    mut transforms: Query<&mut Transform, (With<Obstacles>, Without<setupplayer::Player>)>,
) {
    for mut transform in &mut transforms {
        transform.translation.x = generate_random_int(-200..200) as f32;
    }
}

fn change_background(
    mut players: Query<&mut setupplayer::Player>,
    mut backgrounds: Query<&mut Transform, With<Background>>,
    mut textquery: Query<&mut Text, With<MapText>>,
) {
    let mut current_map = 1;
    for player in players.iter_mut() {
        current_map = player.map;
    }

    for mut transform in &mut backgrounds {
        transform.translation.z = 0.0 as f32;
    }

    let mut i = 1;
    for mut transform in &mut backgrounds {
        if current_map == i {
            transform.translation.z = 1.0 as f32;
        }
        i += 1;
    }

    for mut span in textquery.iter_mut() {
        let value = "Map".to_string() + &current_map.to_string();
        **span = format!("{value}");
    }
}

fn set_line_width(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 1.;

    let (my_config, _) = config_store.config_mut::<MyLargeGizmos>();
    my_config.line_width = 10.;
}
