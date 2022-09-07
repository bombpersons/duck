mod sim;

use bevy_ecs::{world::World, schedule::{Schedule, SystemStage}};
use cgmath::vec3;
use dreamfield_macros::{preprocess_shader_vf, preprocess_shader_vtf};
use dreamfield_renderer::{resources::{ShaderManager, TextureManager, ModelManager}, gl_backend::TextureParams, renderer, components::{Position, Visual, Animation, PlayerCamera}};
use dreamfield_system::{GameHost, WindowSettings, resources::{InputState, SimTime}};

const WINDOW_WIDTH: i32 = 256 * 2;
const WINDOW_HEIGHT: i32 = 256 * 2;
const FIXED_UPDATE: i32 = 30;
const FIXED_UPDATE_TIME: f64 = 1.0 / (FIXED_UPDATE as f64);

// Create the shader manager.
pub fn create_shader_manager() -> ShaderManager {
    ShaderManager::new(vec![
        ("sky", preprocess_shader_vf!(include_bytes!("../resources/shaders/sky.glsl"))),
        ("ps1_no_tess", preprocess_shader_vf!(include_bytes!("../resources/shaders/ps1.glsl"))),
        ("ps1_tess", preprocess_shader_vtf!(include_bytes!("../resources/shaders/ps1.glsl"))),
        ("composite_yiq", preprocess_shader_vf!(include_bytes!("../resources/shaders/composite_yiq.glsl"))),
        ("composite_resolve", preprocess_shader_vf!(include_bytes!("../resources/shaders/composite_resolve.glsl"))),
        ("blit", preprocess_shader_vf!(include_bytes!("../resources/shaders/blit.glsl")))
    ])
}

/// Create the texture manager
pub fn create_texture_manager() -> TextureManager {
    TextureManager::new_with_textures(vec![
        //("sky", (include_bytes!("../resources/textures/sky.png"), TextureParams::repeat_nearest(), true, None))
    ])
}

/// Create the model manager
pub fn create_model_manager() -> ModelManager {
    ModelManager::new_with_models(vec![
        ("level", include_bytes!("../resources/models/level.glb")),
    ])
}

/// Initialise sim, returning the update bevy schedule
fn init_sim(world: &mut World) -> Schedule {
    // Sim resources
    world.insert_resource(InputState::new());
    world.insert_resource(SimTime::new(0.0, FIXED_UPDATE_TIME));

    // Create update schedule
    let mut update_schedule = Schedule::default();

    update_schedule.add_stage("sim", SystemStage::parallel()
        .with_system_set(sim::systems())
    );

    update_schedule
}

/// Initialise renderer, returning the render bevy schedule
fn init_renderer(world: &mut World) -> Schedule {
    // Renderer resources
    world.insert_resource(WindowSettings::with_window_size((WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)));
    world.insert_resource(create_shader_manager());
    world.insert_resource(create_texture_manager());
    world.insert_resource(create_model_manager());

    // Create render schedule
    let mut render_schedule = Schedule::default();

    render_schedule.add_stage("render", SystemStage::single_threaded()
        .with_system_set(renderer::systems())
    );

    render_schedule
}

/// Initialize bevy world
fn init_entities(world: &mut World) {
    // Create sky
    //world.spawn()
    //    .insert(ScreenEffect::new(RunTime::PreScene, "sky", Some("sky")));

    // Create world
    world.spawn()
        .insert(Position::new(vec3(0.0, 0.0, 0.0)))
        .insert(Visual::new("level", true));

    // Create player
    world.spawn()
        // Entrance to village
        .insert(PlayerCamera::new(vec3(0.0, 1.0, 0.0), -0.2, 0.0));
        // Entrance to cathedral
        //.insert(PlayerCamera::new(vec3(-99.988, 6.567, 75.533), -0.0367, 0.8334))
        // In corridor, going out
        //.insert(PlayerCamera::new(vec3(-45.99, 5.75, 17.37), 0.163, 1.7323))
        // Looking at torch
        //.insert(PlayerCamera::new(vec3(-33.04357, 4.42999, 15.564), 0.563, 2.499))
        // Looking at corridor
        //.insert(PlayerCamera::new(vec3(5.2, 0.8, 12.8), 0.03, 2.0))
        // Default dungeon pos
        //.insert(PlayerCamera::new(vec3(0.0, 1.0, 10.0), -0.17, 0.0))
        // Going outside
        //.insert(PlayerCamera::new(vec3(-53.925, 5.8, 19.56), 0.097, 1.57))
        //.insert(PlayerMovement::default());

    // // Create fire orb
    // world.spawn()
    //     .insert(FireOrb::default())
    //     .insert(Position::new(vec3(-9.0, 0.0, 9.0)))
    //     .insert(Visual::new_with_anim("fire_orb", false, Animation::Loop("Orb".to_string())));
}

fn main() {
    // Init logging.
    env_logger::init();
    log::info!("Hello Ducky!");

    // Create the game host.
    let mut host = GameHost::new(WINDOW_WIDTH, WINDOW_HEIGHT, FIXED_UPDATE_TIME);

    // Create the bevy world
    let mut world = World::default();

    // Initialise renderer
    let render_schedule = init_renderer(&mut world);

    // Initialise sim
    let update_schedule = init_sim(&mut world);

    // Initialize entities.
    init_entities(&mut world);

    // Run game
    host.run(world, update_schedule, render_schedule); 
}
