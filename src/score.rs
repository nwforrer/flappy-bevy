use crate::{get_high_score, BirdCollisionEvent, GameState};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_event::<ScoreUpdated>()
            .add_systems(Startup, spawn_score_text)
            .add_systems(OnEnter(GameState::MainMenu), reset_score)
            .add_systems(
                Update,
                (
                    check_score.run_if(in_state(GameState::Playing)),
                    update_score_text,
                ),
            );
    }
}

#[derive(Event)]
struct ScoreUpdated;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
pub struct ScoreArea;

#[derive(Resource, Default)]
pub struct Score {
    pub current: u32,
    pub high: u32,
}

fn reset_score(mut score: ResMut<Score>, mut ev_score_updated: EventWriter<ScoreUpdated>) {
    score.current = 0;
    score.high = get_high_score();
    ev_score_updated.send(ScoreUpdated);
}

fn check_score(
    mut ev_collision: EventReader<BirdCollisionEvent>,
    mut ev_score_updated: EventWriter<ScoreUpdated>,
    query: Query<(Entity, &ScoreArea)>,
    mut score: ResMut<Score>,
) {
    for ev in ev_collision.iter() {
        if query.contains(ev.other) {
            score.current += 1;
            ev_score_updated.send(ScoreUpdated);
        }
    }
}

fn spawn_score_text(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ScoreText,
            ));
        });
}

fn update_score_text(
    ev_score_updated: EventReader<ScoreUpdated>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if !ev_score_updated.is_empty() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("{}", score.current);
        }
    }
}
