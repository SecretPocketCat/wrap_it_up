use bevy::{prelude::*, transform::TransformSystem};

use self::agent::{move_agent, wrap_agent};

pub mod agent;

pub fn agent_plugin(app: &mut App) {
    app.add_system(
        move_agent
            .in_base_set(CoreSet::PostUpdate)
            .before(TransformSystem::TransformPropagate),
    )
    .add_system(
        wrap_agent
            .in_base_set(CoreSet::PostUpdate)
            .after(TransformSystem::TransformPropagate),
    );
}
