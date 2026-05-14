use bevy::prelude::*;
mod game;
mod menu;
mod splash;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GlobalGameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
struct GameSettings {
    brick_rows: usize,
    brick_columns: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            brick_rows: 5,
            brick_columns: 10,
        }
    }
}

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
        .init_state::<GlobalGameState>()
        .init_resource::<GameSettings>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
