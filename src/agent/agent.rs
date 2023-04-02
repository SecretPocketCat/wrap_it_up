use bevy::prelude::*;

use crate::{
    level::level::LevelSize,
    time::time::{ScaledTime, ScaledTimeDelta},
};

#[derive(Component, Deref, Default)]
pub struct Direction(pub Vec2);

#[derive(Component, Deref, Default)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Wrap;

pub(super) fn move_agent(
    mut velocity_q: Query<(&Direction, &Speed, &mut Transform)>,
    time: ScaledTime,
) {
    for (dir, speed, mut trans) in velocity_q.iter_mut() {
        trans.translation += dir.extend(0.) * speed.0 * time.scaled_delta_seconds();
    }
}

// todo: use global pos
pub(super) fn wrap_agent(
    mut wrap_q: Query<(&mut Transform, &GlobalTransform), With<Wrap>>,
    level_size: Res<LevelSize>,
) {
    for (mut t, global_t) in &mut wrap_q.iter_mut() {
        let wrap_diff = t.translation.abs().truncate() - (level_size.0 / 2.);

        if wrap_diff.x > 0. {
            t.translation.x = -t.translation.x + wrap_diff.x * t.translation.x.signum();
        }

        if wrap_diff.y > 0. {
            t.translation.y = -t.translation.y + wrap_diff.y * t.translation.y.signum();
        }
    }
}
