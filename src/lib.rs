mod camera;
mod components;
mod utils;

extern crate js_sys;
extern crate web_sys;

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use components::food;
use components::snake;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(FixedTime::new_from_secs(0.15))
        .insert_resource(snake::LastTailPosition::default())
        .insert_resource(snake::SnakeTails::default())
        .add_event::<snake::GrowthEvent>()
        .add_event::<snake::GameOverEvent>()
        .add_startup_system(camera::setup_camera)
        .add_startup_system(components::snake::spawn_snake)
        .add_system(snake::snake_movement_input.before(snake::snake_movement))
        .add_systems(
            (
                snake::snake_movement,
                snake::snake_eat.after(snake::snake_movement),
                snake::snake_grow.after(snake::snake_eat),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_systems(
            (snake::position_translation, snake::size_scaling).in_base_set(CoreSet::PostUpdate),
        )
        .add_system(food::food_spawner.run_if(on_timer(Duration::from_secs_f32(1.0))))
        .add_system(snake::game_over.after(snake::snake_movement))
        .add_plugins(DefaultPlugins.set({
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake!".to_string(),
                    resolution: bevy::window::WindowResolution::new(500.0, 500.0),
                    ..default()
                }),
                ..default()
            }
        }))
        .run();
}
