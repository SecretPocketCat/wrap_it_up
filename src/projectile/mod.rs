use crate::{
    agent::agent::{Age, DespawnParent, MovementDirection, Rotation, Speed, Wrap},
    time::time::ScaledTime,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};
use seldom_fn_plugin::FnPluginExt;
use std::time::Duration;

pub mod spawner;

pub fn projectile_plugin(app: &mut App) {
    app.fn_plugin(spawner::projectile_spawner_plugin)
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
