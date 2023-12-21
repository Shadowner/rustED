extern crate winit;
use std::sync::Arc;

use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    Version, VulkanLibrary,
};
use winit::event_loop::EventLoop;

use super::engine_window::{init_window, EngineWindow};

pub struct Engine {
    event_loop: EventLoop<()>,
    instance: Arc<Instance>,
    engine_window: EngineWindow,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

impl Engine {
    pub fn init() -> Self {
        let event_loop = EventLoop::new();
        let instance = {
            let library = VulkanLibrary::new().unwrap();
            let extensions = vulkano_win::required_extensions(&library);
            let create_info = InstanceCreateInfo {
                enabled_extensions: extensions,
                enumerate_portability: true,
                max_api_version: Some(Version::V1_1),
                ..Default::default()
            };
            Instance::new(library, create_info)
        }
        .unwrap();

        let engine_window = init_window(&instance);

        Self {
            event_loop,
            instance,
            engine_window,
        }
    }

    pub fn main_loop(&self) {
        
    }
}
