use super::{Projectile, ProjectilePath};
use crate::{
    agent::agent::{Age, DespawnParent, MovementDirection, Rotation, Speed},
    time::time::{ScaledTime, ScaledTimeDelta},
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Duration;

pub fn projectile_spawner_plugin(app: &mut App) {
    app.add_system(spawn_projectiles);
}

// todo: type variant structs
pub enum SpawnerType {
    Straight,
    StopStart,
    Sine,
    Radial,
}

pub enum SpawnInterval {
    Interval(f32),
    IntervalSequence { steps: Vec<f32>, index: usize },
}

#[derive(Component)]
pub struct ProjectileSpawner {
    r#type: SpawnerType,
    interval: SpawnInterval,
    timer: Timer,
    speed: f32,
}

impl ProjectileSpawner {
    pub fn new(
        interval: SpawnInterval,
        r#type: SpawnerType,
        initial_delay: f32,
        speed: f32,
    ) -> Self {
        Self {
            r#type: r#type,
            interval,
            timer: Timer::from_seconds(initial_delay, TimerMode::Once),
            speed,
        }
    }
}

// todo: add a projectile bundle
// todo: dir based on spawner rotation/aim
pub fn spawn_projectiles(
    mut cmd: Commands,
    mut spawner_q: Query<(&mut ProjectileSpawner, &GlobalTransform)>,
    time: ScaledTime,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut spawner, global_t) in spawner_q.iter_mut() {
        spawner.timer.tick(time.scaled_delta());

        if spawner.timer.just_finished() {
            let duration = match &spawner.interval {
                SpawnInterval::Interval(interval) => Duration::from_secs_f32(*interval),
                SpawnInterval::IntervalSequence { steps, index } => {
                    Duration::from_secs_f32(steps[*index])
                }
            };

            spawner.timer.reset();
            spawner.timer.set_duration(duration);

            if let SpawnInterval::IntervalSequence { index, steps } = &mut spawner.interval {
                *index = (*index + 1) % steps.len();
            }

            let mut cmd_e = cmd.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(20.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_translation(global_t.translation()),
                ..default()
            });

            cmd_e.insert(Projectile).insert(Age::default());

            match spawner.r#type {
                SpawnerType::Straight => {
                    cmd_e
                        .insert(MovementDirection(Vec2::X))
                        .insert(Speed(spawner.speed));
                }
                SpawnerType::StopStart => {
                    cmd_e
                        .insert(MovementDirection(Vec2::X))
                        .insert(Speed(spawner.speed))
                        .insert(ProjectilePath::StopStart {
                            time_scale: 4.,
                            initial_direction: Vec2::ZERO,
                        });
                }
                SpawnerType::Sine => {
                    cmd_e
                        .insert(MovementDirection(Vec2::X))
                        .insert(Speed(spawner.speed))
                        .insert(ProjectilePath::SinePath { time_scale: 4. });
                }
                SpawnerType::Radial => {
                    let child = cmd_e
                        .insert(MovementDirection(Vec2::X))
                        .insert(Speed(spawner.speed))
                        .insert(ProjectilePath::StopStart {
                            time_scale: 4.,
                            initial_direction: Vec2::ZERO,
                        })
                        .insert(Speed(30.))
                        .id();

                    let parent = cmd
                        .spawn(SpatialBundle::default())
                        .insert(Rotation(30.))
                        .add_child(child)
                        .id();

                    cmd.entity(child).insert(DespawnParent(parent));
                }
            }
        }
    }
}
