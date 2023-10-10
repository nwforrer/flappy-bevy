use crate::{Scroll, SpriteSize};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CloudSpawnTimer(Timer::from_seconds(0.0, TimerMode::Once)))
            .add_systems(Update, (spawn_cloud,));
    }
}

#[derive(Resource)]
struct CloudSpawnTimer(Timer);

fn spawn_cloud(
    mut commands: Commands,
    mut timer: ResMut<CloudSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let y_pos = rng.gen_range(0.0..400.0);
        let scale = rng.gen_range(1..4) as f32;
        let scroll_speed = rng.gen_range(150.0..200.0);
        let texture = if rand::random() {
            asset_server.load("cloud1.png")
        } else {
            asset_server.load("cloud2.png")
        };
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(1000.0, y_pos, -10.0))
                    .with_scale(Vec3::splat(scale)),
                ..default()
            },
            SpriteSize {
                x: 128.0 * scale,
                y: 64.0 * scale,
            },
            Scroll(scroll_speed),
        ));
        // reset timer to random duration
        let duration = rng.gen_range(1..4);
        timer.0.set_duration(Duration::from_secs(duration));
        timer.0.reset();
    }
}
