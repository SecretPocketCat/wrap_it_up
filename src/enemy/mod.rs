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
    for sign in [-1., 1.].iter() {
        cmd.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::splat(90.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec2::new(350., -270.).extend(0.) * *sign)
                .with_rotation(Quat::from_rotation_z(45f32.to_radians())),
            ..default()
        })
        .insert(MovementDirection::default())
        .insert(Speed(220. * sign))
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
                    -250. * sign,
                ));
        });
    }
}
