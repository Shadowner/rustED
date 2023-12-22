use std::sync::Arc;

use vulkano::{instance::Instance, swapchain::Surface};
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;

pub struct EngineWindow {
    pub surface: Arc<Surface>,
    pub event_loop: EventLoop<()>,
}

pub fn init_window(instance: &Arc<Instance>) -> EngineWindow {
    let event_loop = EventLoop::new();
    let surface = winit::window::WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    EngineWindow {
        surface,
        event_loop,
    }
}
