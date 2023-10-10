use crate::Bird;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(-1380.0))
            .add_event::<BirdCollisionEvent>()
            .add_systems(
                Update,
                (
                    (
                        physics,
                        check_aabb_collision,
                        keep_in_window,
                        rotate_with_velocity,
                    )
                        .run_if(in_state(GameState::Playing)),
                    physics.run_if(in_state(GameState::Dead)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Killable;

#[derive(Component)]
pub struct Collider(pub bool);

#[derive(Component)]
pub struct Rotatable(pub f32);

#[derive(Component)]
pub struct KeepInWindow;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Clone, Copy)]
pub struct SpriteSize {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)]
pub struct Gravity(f32);

#[derive(Event)]
pub struct BirdCollisionEvent {
    pub other: Entity,
}

pub fn physics(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        velocity.y += gravity.0 * time.delta_seconds();

        if velocity.y <= -600.0 {
            velocity.y = -600.0;
        }
    }
}

fn check_aabb_collision(
    mut ev_collision: EventWriter<BirdCollisionEvent>,
    bird_query: Query<(&Transform, &SpriteSize), With<Bird>>,
    mut collider_query: Query<(Entity, &Transform, &SpriteSize, &mut Collider), Without<Bird>>,
) {
    for (bird_transform, bird_size) in &bird_query {
        for (other, other_transform, other_size, mut collider) in &mut collider_query {
            if collider.0
                && collide(
                    bird_transform.translation,
                    Vec2::new(bird_size.x, bird_size.y),
                    other_transform.translation,
                    Vec2::new(other_size.x, other_size.y),
                )
                .is_some()
            {
                collider.0 = false;
                ev_collision.send(BirdCollisionEvent { other });
            }
        }
    }
}

fn keep_in_window(
    window_query: Query<&Window>,
    mut query: Query<(&mut Transform, &SpriteSize), With<KeepInWindow>>,
) {
    let window = window_query.single();
    let window_top = window.resolution.height() / 2.0;
    let window_bottom = -window.resolution.height() / 2.0;
    for (mut transform, sprite_size) in &mut query {
        if transform.translation.y + sprite_size.y / 2.0 > window_top {
            transform.translation.y = window_top - sprite_size.y / 2.0;
        } else if transform.translation.y - sprite_size.y / 2.0 < window_bottom {
            transform.translation.y = window_bottom + sprite_size.y / 2.0;
        }
    }
}

fn rotate_with_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &mut Rotatable)>,
) {
    for (mut transform, velocity, mut rotatable) in &mut query {
        if velocity.y > 0.0 {
            rotatable.0 += 400.0 * time.delta_seconds();
            if rotatable.0 > 20.0 {
                rotatable.0 = 20.0;
            }
        } else if velocity.y < 0.0 {
            rotatable.0 -= 200.0 * time.delta_seconds();
            if rotatable.0 < -70.0 {
                rotatable.0 = -70.0;
            }
        }
        let rotation = Quat::from_rotation_z(rotatable.0.to_radians());
        transform.rotation = rotation;
    }
}
