use bevy::prelude::*;

use crate::{
    BirdCollisionEvent, Collider, GameState, InPlayState, KeepInWindow, Killable, Rotatable,
    ScoreArea, SpriteSize, Velocity,
};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JumpEvent>()
            .add_systems(OnEnter(GameState::MainMenu), init_bird)
            .add_systems(
                Update,
                (
                    (game_input, check_death).run_if(in_state(GameState::Playing)),
                    fly_animation,
                    coast_animation,
                    death_spin.run_if(in_state(GameState::Dead)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Animation {
    pub jump_frame: usize,
    pub idle_frame: usize,
    pub num_idle_frames: usize,
    pub timer: Timer,
}

#[derive(Event)]
pub struct JumpEvent;

fn init_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bird-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let scale = 3.0;
    let sprite_size = SpriteSize {
        x: 32.0 * scale * 0.5,
        y: 32.0 * scale * 0.5,
    };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(scale)),
            ..default()
        },
        Bird,
        Animation {
            idle_frame: 0,
            num_idle_frames: 0,
            jump_frame: 1,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        Killable,
        Velocity { x: 0.0, y: 0.0 },
        Rotatable(0.0),
        KeepInWindow,
        InPlayState,
        sprite_size.clone(),
    ));
}

fn fly_animation(
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
    ev_jump: EventReader<JumpEvent>,
) {
    if !ev_jump.is_empty() {
        for (mut animation, mut sprite) in &mut query {
            animation.timer.reset();
            sprite.index = animation.jump_frame;
        }
    }
}

fn coast_animation(mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut animation, mut sprite) in &mut query {
        if animation.timer.tick(time.delta()).just_finished() {
            sprite.index += 1;
            if sprite.index > animation.num_idle_frames {
                sprite.index = animation.idle_frame;
            }
        }
    }
}

fn game_input(
    mut ev_jump: EventWriter<JumpEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Bird>>,
) {
    let mut velocity = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.y = 420.0;
        ev_jump.send(JumpEvent);
    }
}

fn check_death(
    mut ev_collision: EventReader<BirdCollisionEvent>,
    query: Query<Entity, (With<Collider>, Without<ScoreArea>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in ev_collision.iter() {
        if query.contains(ev.other) {
            next_state.set(GameState::Dead);
        }
    }
}

fn death_spin(time: Res<Time>, mut query: Query<&mut Transform, With<Bird>>) {
    let mut transform = query.single_mut();

    transform.rotate_z(3.0 * time.delta_seconds());
}
