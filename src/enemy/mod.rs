use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};

use crate::{
    agent::agent::{Age, MovementDirection, Speed},
    projectile::{
        spawner::{ProjectileSpawner, SpawnInterval, SpawnerType},
        ProjectilePath,
    },
    state::AppState,
};

pub fn enemy_plugin(app: &mut App) {
    app.add_system(setup_enemies.in_schedule(OnEnter(AppState::Game)));
}

// no procgen for now : just hardcoded enemies to try it out
fn setup_enemies(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(90.)).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec2::new(-280., 400.).extend(0.))
            .with_rotation(Quat::from_rotation_z(45f32.to_radians())),
        ..default()
    })
    .insert(MovementDirection(Vec2::X))
    .insert(Speed(220.))
    .insert(ProjectilePath::SinePath { time_scale: 0.78 })
    .insert(Age::default())
    .with_children(|b| {
        b.spawn(TransformBundle::default())
            .insert(ProjectileSpawner::new(
                SpawnInterval::IntervalSequence {
                    steps: vec![0.3, 0.3, 0.3, 0.3, 0.6],
                    index: 0,
                },
                SpawnerType::Straight,
                1.,
                280.,
            ));
    });

    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(80., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec2::new(100., 400.).extend(0.)),
        ..default()
    })
    .with_children(|b| {
        b.spawn(TransformBundle::default())
            .insert(ProjectileSpawner::new(
                SpawnInterval::IntervalSequence {
                    steps: vec![0.2, 0.2, 0.2, 1.0],
                    index: 0,
                },
                SpawnerType::Sine,
                1.,
                250.,
            ));
    });
}
