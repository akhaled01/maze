mod engine;
mod renderer;
mod resource_manager;
mod player;
mod weapon;
mod audio;
mod maze;
mod level;

fn main() {
    let mut engine = engine::Engine::new();
    engine.run();
}