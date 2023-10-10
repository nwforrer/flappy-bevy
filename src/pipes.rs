use crate::{Collider, GameState, InPlayState, ScoreArea, Scroll, SpriteSize};
use bevy::prelude::*;
use rand::Rng;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                spawn_pipe.run_if(in_state(GameState::Playing)),
                spawn_pipe.run_if(in_state(GameState::Dead)),
            ),
        );
    }
}

#[derive(Component)]
pub struct Pipe;

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

fn spawn_pipe(
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let texture = asset_server.load("pipe.png");
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(-200..200);
        let base = 455.0;
        let scale = 3.0;
        let sprite_size = SpriteSize {
            x: 32.0 * scale,
            y: 256.0 * scale,
        };
        // top pipe
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_scale(Vec3::splat(scale)).with_translation(Vec3::new(
                    700.0,
                    base + offset as f32,
                    0.0,
                )),
                sprite: Sprite {
                    flip_y: false,
                    ..default()
                },
                ..default()
            },
            sprite_size.clone(),
            Scroll(150.0),
            Pipe,
            Collider(true),
            InPlayState,
        ));

        // bottom pipe
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_scale(Vec3::splat(scale)).with_translation(Vec3::new(
                    700.0,
                    -base + offset as f32,
                    0.0,
                )),
                sprite: Sprite {
                    flip_y: true,
                    ..default()
                },
                ..default()
            },
            SpriteSize {
                x: 32.0 * scale,
                y: 256.0 * scale,
            },
            Scroll(150.0),
            Pipe,
            Collider(true),
            InPlayState,
        ));

        commands.spawn((
            TransformBundle {
                local: Transform::from_translation(Vec3::new(700.0, offset as f32, 0.0)),
                ..default()
            },
            Scroll(150.0),
            SpriteSize {
                x: 16.0 * scale,
                y: 32.0 * scale,
            },
            Collider(true),
            ScoreArea,
            InPlayState,
        ));
    }
}
