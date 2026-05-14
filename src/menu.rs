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
