use bevy::prelude::*;

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

mod splash {
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
}

mod menu {
    use super::{GameSettings, GlobalGameState};
    use bevy::prelude::*;

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        #[default]
        Menu,
        Main,
        Settings,
    }

    #[derive(Component)]
    struct Menu;

    #[derive(Component)]
    enum SettingButton {
        RowsInc,
        RowsDec,
        ColsInc,
        ColsDec,
        Back,
        Play,
        Settings,
    }

    #[derive(Component)]
    enum SettingLabel {
        Rows,
        Cols,
    }

    pub fn menu_plugin(app: &mut App) {
        app.add_systems(OnEnter(GlobalGameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                Update,
                (
                    button_system.run_if(in_state(GlobalGameState::Menu)),
                    update_settings_labels.run_if(in_state(GlobalGameState::Menu)),
                ),
            )
            .init_state::<MenuState>();
    }

    fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
        commands
            .spawn((
                Menu,
                DespawnOnExit(GlobalGameState::Menu),
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("BEVY BREAKOUT"),
                    TextFont {
                        font_size: 80.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, menu: Single<Entity, With<Menu>>) {
        commands.entity(menu.entity()).with_children(|parent| {
            parent
                .spawn((
                    DespawnOnExit(MenuState::Main),
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: px(20.0),
                        ..default()
                    },
                ))
                .with_children(|col| {
                    col.spawn((
                        Text::new("MAIN MENU"),
                        TextFont {
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    spawn_button(col, "Play", 150.0, 30.0, SettingButton::Play);
                    spawn_button(col, "Settings", 150.0, 30.0, SettingButton::Settings);
                });
        });
    }

    fn settings_menu_setup(
        mut commands: Commands,
        menu: Single<Entity, With<Menu>>,
        settings: Res<GameSettings>,
    ) {
        commands.entity(menu.entity()).with_children(|parent| {
            parent
                .spawn((
                    DespawnOnExit(MenuState::Settings),
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_content: AlignContent::Center,
                        row_gap: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(23.0)),
                        ..default()
                    },
                ))
                .with_children(|col| {
                    col.spawn((
                        Text::new("SETTINGS"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    spawn_setting_row(
                        col,
                        "Rows",
                        settings.brick_rows,
                        SettingButton::RowsInc,
                        SettingButton::RowsDec,
                        SettingLabel::Rows,
                    );
                    spawn_setting_row(
                        col,
                        "Columns",
                        settings.brick_columns,
                        SettingButton::ColsInc,
                        SettingButton::ColsDec,
                        SettingLabel::Cols,
                    );
                    spawn_button(col, "Back", 100.0, 30.0, SettingButton::Back);
                });
        });
    }

    fn spawn_setting_row(
        parent: &mut ChildSpawnerCommands,
        label: &str,
        value: usize,
        inc_button: SettingButton,
        dec_button: SettingButton,
        label_value: SettingLabel,
    ) {
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new(format!("{label}")),
                    TextFont {
                        font_size: 26.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node {
                        width: Val::Px(100.0),
                        ..default()
                    },
                ));
                spawn_button(row, "-", 36.0, 36.0, dec_button);
                row.spawn((
                    label_value,
                    Text::new(format!("{value}")),
                    TextFont {
                        font_size: 26.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        width: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ));
                spawn_button(row, "+", 36.0, 36.0, inc_button);
            });
    }

    fn spawn_button(
        parent: &mut ChildSpawnerCommands,
        text: &str,
        width: f32,
        height: f32,
        button: SettingButton,
    ) {
        parent
            .spawn((
                button,
                Button,
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                Node {
                    width: Val::Px(width),
                    height: Val::Px(height),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &SettingButton),
            Changed<Interaction>,
        >,
        mut settings: ResMut<GameSettings>,
        mut game_state: ResMut<NextState<GlobalGameState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ) {
        for (interaction, mut color, button) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                    match button {
                        SettingButton::Play => game_state.set(GlobalGameState::Game),
                        SettingButton::Back => menu_state.set(MenuState::Main),
                        SettingButton::Settings => menu_state.set(MenuState::Settings),
                        SettingButton::RowsInc => {
                            settings.brick_rows = (settings.brick_rows + 1).min(10)
                        }
                        SettingButton::RowsDec => {
                            settings.brick_rows = (settings.brick_rows - 1).max(1)
                        }
                        SettingButton::ColsInc => {
                            settings.brick_columns = (settings.brick_columns + 1).min(20)
                        }
                        SettingButton::ColsDec => {
                            settings.brick_columns = (settings.brick_columns - 1).max(1)
                        }
                    }
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                }
                Interaction::None => {
                    *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                }
            }
        }
    }

    fn update_settings_labels(
        settings: Res<GameSettings>,
        mut label_query: Query<(&SettingLabel, &mut Text)>,
    ) {
        if !settings.is_changed() {
            return;
        }
        for (label, mut text) in &mut label_query {
            match label {
                SettingLabel::Rows => **text = settings.brick_rows.to_string(),
                SettingLabel::Cols => **text = settings.brick_columns.to_string(),
            }
        }
    }
}

mod game {
    use super::{GameSettings, GlobalGameState};
    use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
    use bevy::prelude::*;

    const PADDLE_SPEED: f32 = 600.0;
    const PADDLE_WIDTH: f32 = 100.0;
    const BALL_RADIUS: f32 = 10.0;
    const BALL_SPEED: f32 = 300.0;

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum GameState {
        #[default]
        Ready,
        Play,
        Pause,
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
            self.0 = (self.0 * 1.10).clamp_length_max(700.0)
        }
    }

    #[derive(Component, Deref, DerefMut, Debug)]
    struct Velocity(Vec2);

    #[derive(Component)]
    struct Collider;

    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GlobalGameState::Game), game_setup)
            .add_systems(OnEnter(GameState::Pause), pause_overlay)
            .init_state::<GameState>()
            .add_systems(
                Update,
                (
                    toggle_pause.run_if(in_state(GameState::Play).or(in_state(GameState::Pause))),
                    start_game.run_if(in_state(GlobalGameState::Game)),
                ),
            )
            .add_systems(
                FixedUpdate,
                (move_paddle, apply_velocity, check_collision)
                    .chain()
                    .run_if(in_state(GameState::Play)),
            )
            .add_observer(on_collision);
    }

    fn game_setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        window: Single<&Window>,
        settings: Res<GameSettings>,
    ) {
        commands
            .spawn((
                DespawnOnExit(GameState::Ready),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                Node {
                    width: percent(100),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Press space when you are ready"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: 50.0,
                        ..default()
                    },
                ));
            });

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
        let brick_area_width = window.width()
            - (brick_area_gutter * 2.0)
            - (brick_gap * (settings.brick_columns as f32 - 1.0));
        let brick_width = brick_area_width / settings.brick_columns as f32;
        let column_start = -window.width() / 2.0 + brick_area_gutter + brick_width / 2.0;
        let row_start = window.height() / 2.0 - brick_area_gutter - brick_height / 2.0;

        for row in 0..settings.brick_rows {
            let r = rand::random_range(0.0..1.0);
            let g = rand::random_range(0.0..1.0);
            let b = rand::random_range(0.0..1.0);

            for column in 0..settings.brick_columns {
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

    fn start_game(
        mut next_state: ResMut<NextState<GameState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            next_state.set(GameState::Play);
        }
    }

    fn toggle_pause(
        state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            match state.get() {
                GameState::Play => next_state.set(GameState::Pause),
                GameState::Pause => next_state.set(GameState::Play),
                _ => {}
            }
        }
    }

    fn pause_overlay(mut commands: Commands) {
        commands
            .spawn((
                DespawnOnExit(GameState::Pause),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                Node {
                    width: percent(100),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("PAUSED"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: 80.0,
                        ..default()
                    },
                ));
            });
    }

    fn move_paddle(
        mut paddle_transform: Single<&mut Transform, With<Paddle>>,
        window: Single<&Window>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        // camera_query: Single<(&Camera, &GlobalTransform)>,
    ) {
        let paddle_half_width = PADDLE_WIDTH / 2.0;
        let window_half_width = window.width() / 2.0;

        //-------- Move paddle with moues -----------------
        // let (camera, camera_transform) = *camera_query;
        // let Some(cursor_position) = window.cursor_position() else {
        //     return;
        // };
        // let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        //     return;
        // };

        // paddle_transform.translation.x = point.x.clamp(
        //     -window_half_width + paddle_half_width,
        //     window_half_width - paddle_half_width,
        // )

        //----------- Move paddle with key input -------------
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

    fn apply_velocity(
        ball_query: Single<(&mut Transform, &Velocity), With<Ball>>,
        time: Res<Time>,
    ) {
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
}
