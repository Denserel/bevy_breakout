use super::GlobalGameState;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GlobalGameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GlobalGameState::Splash)));
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bevy_logo = asset_server.load("bevy_logo_bevy.png");

    commands.spawn((
        DespawnOnExit(GlobalGameState::Splash),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(ImageNode::new(bevy_logo))],
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GlobalGameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(GlobalGameState::Menu);
    }
}
