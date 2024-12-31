use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};

const PADDLE_SPEED: f32 = 400.0;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 0.95)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddle)
        .run();
}

#[derive(Component)]
struct Paddle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Paddle,
        Mesh2d(meshes.add(Rectangle::new(80.0, 20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            ..default()
        },
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // mut query: Query<(&mut Transform, With<Paddle>)>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
){
    if keyboard_input.pressed(KeyCode::KeyA) {
        paddle_transform.translation.x = paddle_transform.translation.x - PADDLE_SPEED * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        paddle_transform.translation.x = paddle_transform.translation.x + PADDLE_SPEED * time.delta_secs();
    }
}