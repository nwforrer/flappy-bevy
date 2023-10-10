use bevy::prelude::*;

use crate::{Bird, GameState, SpriteSize};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), create_debug_rects)
            .add_systems(Update, debug_follow_bird);
    }
}

#[derive(Component)]
pub struct DebugSpriteCollider;

fn create_debug_rects(mut commands: Commands, query: Query<&SpriteSize, With<Bird>>) {
    if let Ok(size) = query.get_single() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..default()
                },
                ..default()
            },
            DebugSpriteCollider,
        ));
    }
}

fn debug_follow_bird(
    bird_query: Query<&Transform, With<Bird>>,
    mut debug_query: Query<&mut Transform, (With<DebugSpriteCollider>, Without<Bird>)>,
) {
    if let Ok(bird_transform) = bird_query.get_single() {
        for mut debug_transform in debug_query.iter_mut() {
            debug_transform.translation = bird_transform.translation;
        }
    }
}
