use std::sync::Arc;

use vulkano::{render_pass::Framebuffer, swapchain::SwapchainAcquireFuture};

pub struct ImageRelated {
    pub previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
    pub image_index: u32,
    pub future: SwapchainAcquireFuture,
    pub framebuffer: Arc<Framebuffer>,
}
