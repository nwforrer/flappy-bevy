use crate::{BirdCollisionEvent, Collider, JumpEvent, Pipe, ScoreArea};
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalVolume::new(0.5))
            .add_systems(Update, (jump_sound, hit_sound, score_sound));
    }
}

fn jump_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_jump: EventReader<JumpEvent>,
) {
    if !ev_jump.is_empty() {
        ev_jump.clear();
        commands.spawn(AudioBundle {
            source: asset_server.load("jump.ogg"),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
    }
}

fn hit_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_hit: EventReader<BirdCollisionEvent>,
    query: Query<Entity, (With<Collider>, With<Pipe>)>,
) {
    for ev in ev_hit.iter() {
        if query.contains(ev.other) {
            commands.spawn(AudioBundle {
                source: asset_server.load("hit.ogg"),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        }
    }
}

fn score_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_hit: EventReader<BirdCollisionEvent>,
    query: Query<Entity, With<ScoreArea>>,
) {
    for ev in ev_hit.iter() {
        if query.contains(ev.other) {
            commands.spawn(AudioBundle {
                source: asset_server.load("score.ogg"),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        }
    }
}
