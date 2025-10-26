use bevy::color::palettes::css::{BLUE, PURPLE, RED};
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use bevy::window::WindowMode;

const PADDLE_SPEED: f32 = 600.0;
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
                resizable: false,
                // position: WindowPosition::Centered(MonitorSelection::Primary),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (check_collision, apply_velocity, move_paddle))
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

#[derive(Event, Debug)]
struct CollisionEvent;
#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Paddle,
        Collider,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Vec3::new(0.0, -window.height() / 2.0 + 50.0, 0.0),
            scale: Vec3::new(PADDLE_WIDTH, 22.0, 1.0),
            ..default()
        },
    ));

    commands.spawn((
        Ball,
        Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(PURPLE))),
        Transform {
            translation: Vec3::new(0.0, -window.height() / 2.0 + 70.0, 0.0),
            scale: Vec2::splat(BALL_RADIUS * 2.0).extend(1.0),
            ..default()
        },
    ));

    let brick_area_gutter = 10.0;
    let brick_gap = 5.0;
    let brick_height = 20.0;
    let brick_area_width =
        window.width() - (brick_area_gutter * 2.0) - (brick_gap * (BRICK_COLUMNS as f32 - 1.0));
    let brick_width = brick_area_width / BRICK_COLUMNS as f32;
    let column_start = -window.width() / 2.0 + brick_area_gutter + brick_width / 2.0;
    let row_start = window.height() / 2.0 - brick_area_gutter - brick_height / 2.0;

    for row in 0..BRICK_ROWS {
        for column in 0..BRICK_COLUMNS {
            let brick_x = column_start + column as f32 * (brick_width + brick_gap);
            let brick_y = row_start - row as f32 * (brick_height + brick_gap);
            commands.spawn((
                Brick,
                Collider,
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
                Transform {
                    translation: Vec3::new(brick_x, brick_y, 0.0),
                    scale: Vec3::new(brick_width, brick_height, 1.0),
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
    // Move paddle with moues
    // let (camera, camera_transform) = *camera_query;
    // let Some(cursor_position) = window.cursor_position() else { return; };
    // let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };
    // if point.x < (window.width() / 2.0 - PADDLE_WIDTH / 2.0) && point.x > (-window.width() / 2.0 + PADDLE_WIDTH / 2.0) {
    //      paddle_transform.translation.x = point.x;
    // }

    // Move paddle with keys
    // if keyboard_input.pressed(KeyCode::KeyD)
    //     && (paddle_transform.translation.x < ((window.width() / 2.0) - PADDLE_WIDTH / 2.0))
    // {
    //     paddle_transform.translation.x =
    //         paddle_transform.translation.x + PADDLE_SPEED * time.delta_secs();
    // }

    // if keyboard_input.pressed(KeyCode::KeyA)
    //     && (paddle_transform.translation.x > ((-window.width() / 2.0) + PADDLE_WIDTH / 2.0))
    // {
    //     paddle_transform.translation.x =
    //         paddle_transform.translation.x - PADDLE_SPEED * time.delta_secs();
    // }

    let paddle_half_width = PADDLE_WIDTH / 2.0;
    let window_half_width = window.width() / 2.0;

    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let paddle_new_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();
    paddle_transform.translation.x = paddle_new_position.clamp(
        -window_half_width + paddle_half_width,
        window_half_width - paddle_half_width,
    );
}

fn move_ball(
    ball_query: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    // for (mut ball_transform, mut ball_velocity) in ball_query {

    //     // ToDo: move collision detection to event
    //     // Basic window collision detection
    //     if ball_transform.translation.x + BALL_RADIUS >= window.width() / 2.0 || ball_transform.translation.x - BALL_RADIUS <= -window.width() / 2.0 {
    //         ball_velocity.x *= -1.0;
    //     }

    //     if ball_transform.translation.y + BALL_RADIUS >= window.height() / 2.0 || ball_transform.translation.y - BALL_RADIUS <= -window.height() / 2.0 {
    //         ball_velocity.y *= -1.0;
    //     }

    //     ball_transform.translation.x += ball_velocity.x * time.delta_secs();
    //     ball_transform.translation.y += ball_velocity.y * time.delta_secs();
    // }

    let (mut ball_transform, mut ball_velocity) = ball_query.into_inner();
    // ToDo: move collision detection to event
    // Basic window collision detection
    if ball_transform.translation.x + BALL_RADIUS >= window.width() / 2.
        || ball_transform.translation.x - BALL_RADIUS <= -window.width() / 2.0
    {
        ball_velocity.x *= -1.0;
    }

    if ball_transform.translation.y + BALL_RADIUS >= window.height() / 2.0
        || ball_transform.translation.y - BALL_RADIUS <= -window.height() / 2.0
    {
        ball_velocity.y *= -1.0;
    }

    // ball_transform.translation.x += ball_velocity.x * time.delta_secs();
    // ball_transform.translation.y += ball_velocity.y * time.delta_secs();
    ball_transform.translation +=
        Vec3::new(ball_velocity.x, ball_velocity.y, 0.0) * time.delta_secs();
}

fn apply_velocity(
    ball_query: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    time: Res<Time>,
) {
    let (mut ball_transform, ball_velocity) = ball_query.into_inner();
    ball_transform.translation +=
        Vec3::new(ball_velocity.x, ball_velocity.y, 0.0) * time.delta_secs();
}

fn check_collision(
    mut commands: Commands,
    ball_query: Single<(&Transform, &mut Velocity), With<Ball>>,
    window: Single<&Window>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.into_inner();

    if ball_transform.translation.x + BALL_RADIUS >= window.width() / 2.
        || ball_transform.translation.x - BALL_RADIUS <= -window.width() / 2.0
    {
        ball_velocity.x *= -1.0;
    }

    if ball_transform.translation.y + BALL_RADIUS >= window.height() / 2.0
        || ball_transform.translation.y - BALL_RADIUS <= -window.height() / 2.0
    {
        ball_velocity.y *= -1.0;
    }

    for (entity, transform, brick) in &collider_query {
        let ball_box = BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);
        let entity_box = Aabb2d::new(
            transform.translation.truncate(),
            transform.scale.truncate() / 2.0,
        );

        if ball_box.intersects(&entity_box) {
            let closest = entity_box.closest_point(ball_box.center());
            let offset = ball_box.center() - closest;

            if offset.x.abs() >= offset.y.abs() {
                ball_velocity.x *= -1.0;
            } else {
                ball_velocity.y *= -1.0;
            }

            if brick.is_some() {
                commands.entity(entity).despawn();
            }
        }
    }
}
