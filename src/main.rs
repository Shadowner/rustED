use core::engine_core::Engine;

extern crate vulkano;
mod core;

fn main() {
    let (mut engine, mut engine_window) = core::engine_core::Engine::init();
    engine.start_main_loop(engine_window, render_loop);
}

fn render_loop(engine: &mut Engine) {}
