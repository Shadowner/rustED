use core::{engine_core::Engine, image_related};

use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    format::ClearValue,
    swapchain::SwapchainPresentInfo,
    sync::{self as vulkano_sync, FlushError, GpuFuture},
};
use rand::{self, random};

extern crate vulkano;
mod core;
mod geometry;

fn main() {
    let (mut engine, mut engine_window) = core::engine_core::Engine::init();
    engine.start_main_loop(engine_window, render_loop);
}

fn render_loop(
    engine: &mut Engine,
    mut image_related: image_related::ImageRelated,
) -> Option<Box<dyn GpuFuture>> {


    let clear_values: Vec<Option<ClearValue>> = vec![Some([random(), random(), random(), 0.0].into())];

    let mut cmd_buffer_builder = AutoCommandBufferBuilder::primary(
        &engine.command_buffer_allocator,
        engine.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Failed to create command buffer builder");

    cmd_buffer_builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values,
                ..RenderPassBeginInfo::framebuffer(image_related.framebuffer.clone())
            },
            SubpassContents::Inline,
        )
        .expect("Failed to begin render pass")
        .end_render_pass()
        .expect("Failed to end render pass");

    let command_buffer = cmd_buffer_builder.build().unwrap();

    let future = image_related
        .previous_frame_end
        .take()
        .unwrap()
        .join(image_related.future)
        .then_execute(engine.queue.clone(), command_buffer)
        .unwrap()
        .then_swapchain_present(
            engine.queue.clone(),
            SwapchainPresentInfo::swapchain_image_index(
                engine.swapchain_related.swapchain.clone(),
                image_related.image_index,
            ),
        )
        .then_signal_fence_and_flush();

    match future {
        Ok(future) => {
            return Some(Box::new(future) as Box<_>);
        }
        Err(FlushError::OutOfDate) => {
            engine.swapchain_related.recreate_swapchain = true;
            return Some(Box::new(vulkano_sync::now(engine.device.clone())) as Box<_>);
        }
        Err(e) => {
            println!("Failed to flush future: {:?}", e);
            return Some(Box::new(vulkano_sync::now(engine.device.clone())) as Box<_>);
        }
    }
    //clear console
}
