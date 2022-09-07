use bevy_ecs::schedule::SystemSet;

// Sim systems
pub fn systems() -> SystemSet {
    SystemSet::new()
        .label("sim")
}