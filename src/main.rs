use bevy::color::palettes::css::{BLUE, PURPLE, RED};
use bevy::prelude::*;
use bevy::window::WindowMode;

const PADDLE_SPEED: f32 = 400.0;
const PADDLE_WIDTH: f32 = 100.0;
const BRICK_ROWS: usize = 5;
const BRICK_COLUMNS: usize = 20;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Breakout".to_string(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_paddle)
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Paddle,
        Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, 20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Vec3::new(0.0, -window.height() / 2.0 + 50.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(PURPLE))),
        Transform {
            translation: Vec3::new(0.0, -window.height() / 2.0 + 80.0, 0.0),
            ..default()
        },
    ));

    let brick_area_gutter = 10.0;
    let brick_gap = 5.0;
    let brick_height = 20.0;
    let brick_area_width =
        window.width() - brick_area_gutter * 2.0 - brick_gap * (BRICK_COLUMNS as f32 - 1.0);
    let brick_width = brick_area_width / BRICK_COLUMNS as f32;
    let column_start = -window.width() / 2.0 + brick_area_gutter + brick_width / 2.0;
    let row_start = window.height() / 2.0 - brick_area_gutter - brick_height / 2.0;

    for row in 0..BRICK_ROWS {
        for column in 0..BRICK_COLUMNS {
            let brick_x = column_start + column as f32 * (brick_width + brick_gap);
            let brick_y = row_start - row as f32 * (brick_height + brick_gap);
            commands.spawn((
                Brick,
                Mesh2d(meshes.add(Rectangle::new(brick_width, brick_height))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
                Transform {
                    translation: Vec3::new(brick_x, brick_y, 0.0),
                    ..default()
                },
            ));
        }
    }
}

fn move_paddle(
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    window: Single<&Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    // let (camera, camera_transform) = *camera_query;
    // let Some(cursor_position) = window.cursor_position() else { return; };
    // let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };
    // if point.x < (window.width() / 2.0 - PADDLE_WIDTH / 2.0) && point.x > (-window.width() / 2.0 + PADDLE_WIDTH / 2.0) {
    //      paddle_transform.translation.x = point.x;
    // }

    if keyboard_input.pressed(KeyCode::KeyD)
        && (paddle_transform.translation.x < ((window.width() / 2.0) - PADDLE_WIDTH / 2.0))
    {
        paddle_transform.translation.x =
            paddle_transform.translation.x + PADDLE_SPEED * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyA)
        && (paddle_transform.translation.x > ((-window.width() / 2.0) + PADDLE_WIDTH / 2.0))
    {
        paddle_transform.translation.x =
            paddle_transform.translation.x - PADDLE_SPEED * time.delta_secs();
    }
}
