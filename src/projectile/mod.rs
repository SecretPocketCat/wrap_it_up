use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};

use crate::agent::agent::{MovementDirection, Speed};

pub fn projectile_plugin(app: &mut App) {
    app.add_system(test_projectile_spawning.run_if(on_timer(Duration::from_secs_f32(0.5))));
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct WavyPath;

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
    .insert(MovementDirection(Vec2::X))
    .insert(Speed(300.));

    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(-300., -200., 0.)),
        ..default()
    })
    .insert(Projectile)
    .insert(WavyPath)
    .insert(MovementDirection(Vec2::X))
    .insert(Speed(300.));
}

pub(super) fn sine_path(mut dir_q: Query<(&mut MovementDirection, &WavyPath)>) {
    for (mut dir, path) in &mut dir_q {}
}
