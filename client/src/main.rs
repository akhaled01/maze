mod engine;
mod renderer;
mod resource_manager;
mod player;
mod weapon;
mod audio;

fn main() {
    let mut engine = engine::Engine::new();
    engine.run();
}