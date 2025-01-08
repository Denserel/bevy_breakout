use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};

const PADDLE_SPEED: f32 = 400.0;
const PADDLE_WIDTH: f32 = 100.0;

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
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>
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
}

fn move_paddle(
    // keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    // time: Res<Time>,
){
    let (camera, camera_transform) = *camera_query;
    let Some(cursor_position) = window.cursor_position() else { return; };
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    // if keyboard_input.pressed(KeyCode::KeyD) && (paddle_transform.translation.x < ((window.width() / 2.0) - PADDLE_WIDTH / 2.0)) {
    //     paddle_transform.translation.x = paddle_transform.translation.x + PADDLE_SPEED * time.delta_secs();
    // }

    // if keyboard_input.pressed(KeyCode::KeyA) && (paddle_transform.translation.x > ((-window.width() / 2.0) + PADDLE_WIDTH / 2.0)){
    //     paddle_transform.translation.x = paddle_transform.translation.x - PADDLE_SPEED * time.delta_secs();
    // }


    if point.x < ((window.width() / 2.0) - PADDLE_WIDTH / 2.0) && point.x > ((-window.width() / 2.0) + PADDLE_WIDTH / 2.0) {
         paddle_transform.translation.x = point.x;
    }

}