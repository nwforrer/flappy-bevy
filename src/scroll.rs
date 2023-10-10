use bevy::prelude::*;

use crate::SpriteSize;

pub struct ScrollPlugin;

impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (scroll, remove_scrolling_object, scroll_wrap).chain(),
        );
    }
}

#[derive(Component)]
pub struct Scroll(pub f32);

#[derive(Component)]
pub struct ScrollWrap;

fn scroll(time: Res<Time>, mut query: Query<(&mut Transform, &Scroll)>) {
    for (mut transform, scroll) in query.iter_mut() {
        transform.translation.x -= scroll.0 * time.delta_seconds();
    }
}

fn remove_scrolling_object(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<(Entity, &Transform, &SpriteSize), (With<Scroll>, Without<ScrollWrap>)>,
) {
    let window = window_query.single();
    let window_left = -window.resolution.width() / 2.0;
    for (entity, transform, sprite_size) in &query {
        if transform.translation.x + sprite_size.x / 2.0 < window_left {
            commands.entity(entity).despawn();
        }
    }
}

fn scroll_wrap(
    window_query: Query<&Window>,
    mut query: Query<(&mut Transform, &SpriteSize), (With<Scroll>, With<ScrollWrap>)>,
) {
    let window = window_query.single();
    let window_left = -window.resolution.width() / 2.0;
    for (mut transform, sprite_size) in &mut query {
        if transform.translation.x + sprite_size.x / 2.0 < window_left {
            transform.translation.x += sprite_size.x * 2.0;
        }
    }
}
