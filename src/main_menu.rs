use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(
                Update,
                (main_menu_input).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

#[derive(Component)]
struct MenuText;

fn spawn_menu(mut commands: Commands) {
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
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuText>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn main_menu_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}
