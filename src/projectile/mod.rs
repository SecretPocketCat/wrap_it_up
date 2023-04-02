use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};

use crate::{
    agent::agent::{Age, DespawnParent, MovementDirection, Rotation, Speed, Wrap},
    time::time::ScaledTime,
};

pub fn projectile_plugin(app: &mut App) {
    app.add_system(test_projectile_spawning.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .add_system(apply_projectile_path);
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub enum ProjectilePath {
    StopStart {
        time_scale: f32,
        initial_direction: Vec2,
    },
    SinePath {
        time_scale: f32,
    },
}

// todo: add a projectile bundle
pub fn test_projectile_spawning(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(-300., -200., 0.)),
        ..default()
    })
    .insert(Projectile)
    .insert(Age::default())
    .insert(MovementDirection(Vec2::X))
    .insert(Speed(300.));

    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(-300., 200., 0.)),
        ..default()
    })
    .insert(Projectile)
    .insert(Age::default())
    .insert(ProjectilePath::StopStart {
        time_scale: 4.,
        initial_direction: Vec2::X,
    })
    .insert(MovementDirection(Vec2::X))
    .insert(Speed(300.));

    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
        ..default()
    })
    .insert(Projectile)
    .insert(Age::default())
    .insert(ProjectilePath::SinePath { time_scale: 4. })
    .insert(MovementDirection(Vec2::X))
    .insert(Speed(300.));

    cmd.spawn(SpatialBundle::default())
        .insert(Rotation(30.))
        .with_children(|b| {
            let e = b.parent_entity();

            b.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(20.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                ..default()
            })
            .insert(Projectile)
            .insert(MovementDirection(Vec2::X))
            .insert(Speed(30.))
            .insert(Age::default())
            .insert(DespawnParent(e));
        });

    // cmd.spawn(SpatialBundle::default())
    //     .insert(Rotation(30.))
    //     .with_children(|b| {
    //         b.spawn(SpatialBundle::default())
    //             .insert(MovementDirection(Vec2::X))
    //             .insert(Speed(30.))
    //             .with_children(|b| {
    //                 b.spawn(MaterialMesh2dBundle {
    //                     mesh: meshes.add(shape::Circle::new(20.).into()).into(),
    //                     material: materials.add(ColorMaterial::from(Color::YELLOW)),
    //                     ..default()
    //                 })
    //                 .insert(Projectile)
    //                 .insert(SinePath { time_scale: 5. })
    //                 .insert(MovementDirection(Vec2::ZERO))
    //                 .insert(Speed(200.))
    //                 .insert(Age::default());
    //             });
    //     });
}

fn apply_projectile_path(mut dir_q: Query<(&mut MovementDirection, &ProjectilePath, &Age)>) {
    for (mut dir, path, age) in &mut dir_q {
        match path {
            ProjectilePath::StopStart {
                time_scale,
                initial_direction,
            } => {
                dir.0 = *initial_direction * (age.0 * time_scale).sin().abs();
            }
            ProjectilePath::SinePath { time_scale } => {
                dir.0 = Vec2::new(dir.x, (age.0 * time_scale).sin());
            }
        }
    }
}
