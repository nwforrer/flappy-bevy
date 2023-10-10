use bevy::prelude::*;

use crate::{GameState, InPlayState, Score};

pub struct DeathMenuPlugin;

impl Plugin for DeathMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputDelayTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .add_systems(OnEnter(GameState::Dead), (spawn_menu, reset_delay_timer))
            .add_systems(Update, death_menu_input.run_if(in_state(GameState::Dead)))
            .add_systems(OnExit(GameState::Dead), (despawn_menu, despawn_play_state));
    }
}

#[derive(Component)]
struct MenuText;

#[derive(Resource)]
struct InputDelayTimer(Timer);

#[derive(Default)]
struct DeathInputActive(bool);

fn reset_delay_timer(mut input_delay_timer: ResMut<InputDelayTimer>) {
    input_delay_timer.0.reset();
}

fn spawn_menu(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        TextBundle::from_section(
            "Press SPACE",
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            ..default()
        }),
        MenuText,
    ));

    let high_score_text = if score.current > score.high {
        "New High Score!".to_string()
    } else {
        format!("High score: {}", score.high)
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            MenuText,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                high_score_text,
                TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuText>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn despawn_play_state(mut commands: Commands, query: Query<Entity, With<InPlayState>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn death_menu_input(
    mut input_active: Local<DeathInputActive>,
    mut input_delay_timer: ResMut<InputDelayTimer>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input_delay_timer.0.tick(time.delta()).just_finished() {
        input_active.0 = true;
    }

    if input_active.0 {
        if keyboard_input.just_pressed(KeyCode::Space) {
            input_active.0 = false;
            next_state.set(GameState::MainMenu);
        }
    }
}
