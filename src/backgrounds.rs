use crate::{Scroll, ScrollWrap, SpriteSize};
use bevy::prelude::*;

pub struct BackgroundsPlugin;

impl Plugin for BackgroundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_backgrounds);
    }
}

fn spawn_backgrounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mountains1 = asset_server.load("mountains1.png");
    let mountains2 = asset_server.load("mountains2.png");

    for i in 0..=1 {
        commands.spawn((
            SpriteBundle {
                texture: mountains1.clone(),
                transform: Transform::from_translation(Vec3::new(i as f32 * 800., 0.0, -5.0)),
                ..default()
            },
            SpriteSize { x: 800., y: 600. },
            Scroll(100.),
            ScrollWrap,
        ));
        commands.spawn((
            SpriteBundle {
                texture: mountains2.clone(),
                transform: Transform::from_translation(Vec3::new(i as f32 * 800., 30.0, -10.0)),
                ..default()
            },
            SpriteSize { x: 800., y: 600. },
            Scroll(30.),
            ScrollWrap,
        ));
    }
}
