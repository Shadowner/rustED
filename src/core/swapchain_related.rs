use std::sync::Arc;

use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError};
use winit::window::Window;

use super::{
    engine_core::{self},
    engine_window,
};

pub struct SwapchainRelated {
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<vulkano::image::SwapchainImage>>,
}

impl SwapchainRelated {
    pub fn recreate_swapchain(&mut self, surface: &Arc<Surface>) {
        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
        let image_extent: [u32; 2] = window.inner_size().into();

        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent,
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        self.swapchain = new_swapchain;
        self.images = new_images;
    }
}
