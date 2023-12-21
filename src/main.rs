extern crate vulkano;
mod core;

fn main() {
    let mut engine = core::engine::Engine::init();
    engine.main_loop();
}
