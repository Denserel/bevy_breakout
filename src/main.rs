use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use bevy::window::WindowMode;
use rand::Rng;

const PADDLE_SPEED: f32 = 600.0;
const PADDLE_WIDTH: f32 = 100.0;
const BRICK_ROWS: usize = 10;
const BRICK_COLUMNS: usize = 20;
const BALL_RADIUS: f32 = 10.0;
const BALL_SPEED: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Breakout".to_string(),
                resizable: false,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        .insert_resource(Time::<Fixed>::from_hz(120.0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (move_paddle, apply_velocity, check_collision).chain(),
        )
        .add_observer(on_collision)
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

#[derive(EntityEvent)]
struct CollisionEvent {
    pub entity: Entity,
    pub nudge: Vec2,
}

impl Velocity {
    fn accelerate(&mut self) {
        self.0 = (self.0 * 1.10).clamp_length_max(800.0)
    }
}

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

    commands
        .spawn((
            Paddle,
            Collider,
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.6, 0.2, 0.2))),
            Transform {
                translation: Vec3::new(0.0, -window.height() / 2.0 + 50.0, 0.0),
                scale: Vec3::new(PADDLE_WIDTH, 22.0, 1.0),
                ..default()
            },
        ))
        .observe(on_paddle_collision);

    commands
        .spawn((
            Ball,
            Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)),
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.6, 0.1, 0.5))),
            Transform {
                translation: Vec3::new(0.0, -window.height() / 2.0 + 70.0, 0.0),
                scale: Vec2::splat(BALL_RADIUS * 2.0).extend(1.0),
                ..default()
            },
        ))
        .observe(on_ball_collision);

    let brick_area_gutter = 10.0;
    let brick_gap = 5.0;
    let brick_height = 20.0;
    let brick_area_width =
        window.width() - (brick_area_gutter * 2.0) - (brick_gap * (BRICK_COLUMNS as f32 - 1.0));
    let brick_width = brick_area_width / BRICK_COLUMNS as f32;
    let column_start = -window.width() / 2.0 + brick_area_gutter + brick_width / 2.0;
    let row_start = window.height() / 2.0 - brick_area_gutter - brick_height / 2.0;

    for row in 0..BRICK_ROWS {
        let r = rand::rng().random_range(0.0..1.0);
        let g = rand::rng().random_range(0.0..1.0);
        let b = rand::rng().random_range(0.0..1.0);
        for column in 0..BRICK_COLUMNS {
            let brick_x = column_start + column as f32 * (brick_width + brick_gap);
            let brick_y = row_start - row as f32 * (brick_height + brick_gap);
            commands
                .spawn((
                    Brick,
                    Collider,
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::srgb(r, g, b))),
                    Transform {
                        translation: Vec3::new(brick_x, brick_y, 0.0),
                        scale: Vec3::new(brick_width, brick_height, 1.0),
                        ..default()
                    },
                ))
                .observe(on_brick_collision);
        }
    }
}

fn move_paddle(
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    window: Single<&Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let paddle_half_width = PADDLE_WIDTH / 2.0;
    let window_half_width = window.width() / 2.0;

    // Move paddle with moues
    let (camera, camera_transform) = *camera_query;
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    paddle_transform.translation.x = point.x.clamp(
        -window_half_width + paddle_half_width,
        window_half_width - paddle_half_width,
    )

    // let mut direction = 0.0;
    // if keyboard_input.pressed(KeyCode::KeyA) {
    //     direction -= 1.0;
    // }
    // if keyboard_input.pressed(KeyCode::KeyD) {
    //     direction += 1.0;
    // }

    // let paddle_new_position =
    //     paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();
    // paddle_transform.translation.x = paddle_new_position.clamp(
    //     -window_half_width + paddle_half_width,
    //     window_half_width - paddle_half_width,
    // );
}

fn apply_velocity(ball_query: Single<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let (mut ball_transform, ball_velocity) = ball_query.into_inner();

    ball_transform.translation +=
        Vec3::new(ball_velocity.x, ball_velocity.y, 0.0) * time.delta_secs();
}

fn check_collision(
    mut commands: Commands,
    window: Single<&Window>,
    ball_query: Single<(&Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Paddle>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.into_inner();
    let window_half_with = window.width() / 2.0;
    let window_half_height = window.height() / 2.0;

    if ball_transform.translation.x + BALL_RADIUS >= window_half_with {
        ball_velocity.x = -ball_velocity.x.abs();
    } else if ball_transform.translation.x - BALL_RADIUS <= -window_half_with {
        ball_velocity.x = ball_velocity.x.abs();
    }

    if ball_transform.translation.y + BALL_RADIUS >= window_half_height {
        ball_velocity.y = -ball_velocity.y.abs()
    } else if ball_transform.translation.y - BALL_RADIUS <= -window_half_height {
        ball_velocity.y = ball_velocity.y.abs()
    }

    let ball_bounding_circle =
        BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS);

    for (entity, transform) in &collider_query {
        let collision_entity_bounding_box = Aabb2d::new(
            transform.translation.truncate(),
            transform.scale.truncate() / 2.0,
        );

        if ball_bounding_circle.intersects(&collision_entity_bounding_box) {
            let closest =
                collision_entity_bounding_box.closest_point(ball_bounding_circle.center());
            let offset = ball_bounding_circle.center() - closest;
            let distance = offset.length();
            let normal = if offset == Vec2::ZERO {
                Vec2::Y
                // (ball_bounding_circle.center() - collision_entity_bounding_box.center()).normalize_or_zero()
            } else {
                offset / distance
                // offset.normalize()
            };

            let overlap = BALL_RADIUS - distance;
            let nudge = normal * overlap;

            if normal.x.abs() > normal.y.abs() {
                ball_velocity.x = ball_velocity.x.abs() * normal.x.signum()
            } else {
                ball_velocity.y = ball_velocity.y.abs() * normal.y.signum()
            }

            if paddle_query.get(entity).is_ok() {
                let paddle_relative_impact_point = (ball_bounding_circle.center().x
                    - collision_entity_bounding_box.center().x)
                    / (PADDLE_WIDTH / 2.0);

                let speed = ball_velocity.length();
                let new_x = paddle_relative_impact_point * 0.8;
                let new_direction = Vec2::new(new_x, 1.0).normalize();

                ball_velocity.0 = new_direction * speed;
            }

            commands.trigger(CollisionEvent { entity, nudge });
        }
    }
}

fn on_brick_collision(collision: On<CollisionEvent>, mut commands: Commands) {
    let entity = collision.entity;
    commands.entity(entity).despawn();
}

fn on_ball_collision(
    collision: On<CollisionEvent>,
    mut ball_transform: Single<&mut Transform, With<Ball>>,
) {
    ball_transform.translation += collision.nudge.extend(0.0)
}

fn on_paddle_collision(_collision: On<CollisionEvent>) {}

fn on_collision(
    _collision: On<CollisionEvent>,
    mut ball_velocity: Single<&mut Velocity, With<Ball>>,
) {
    ball_velocity.accelerate();
}
