extern crate winit;
use std::sync::Arc;

use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo},
    image::{view::ImageView, ImageAccess, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{Swapchain, SwapchainCreateInfo},
    sync::{self, GpuFuture},
    Version, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use super::{
    engine_window::{self, init_window, EngineWindow},
    physical_devices::{get_compatible_physical_devices, get_prefered_physical_device},
    swapchain_related::SwapchainRelated,
};

pub struct Engine {
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub command_buffer_allocator: StandardCommandBufferAllocator,
    pub viewport: Viewport,
    pub swapchain_related: SwapchainRelated,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

impl Engine {
    pub fn init() -> (Self, EngineWindow) {
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
        .expect(" Impossible to create a Vulkan instance");

        let device_requirement = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let engine_window = init_window(&instance);
        let physical_devices =
            get_compatible_physical_devices(&instance, &engine_window.surface, &device_requirement);
        let physical_device = get_prefered_physical_device(physical_devices);

        let (device, mut queues) = Device::new(
            physical_device.0,
            DeviceCreateInfo {
                enabled_extensions: device_requirement,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: physical_device.1,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("Impossible to create a device");

        let (mut swapchain, images) = {
            let caps = device
                .physical_device()
                .surface_capabilities(&engine_window.surface, Default::default())
                .expect("Failed to get surface capabilities");

            let usage = caps.supported_usage_flags;
            let alpha = caps
                .supported_composite_alpha
                .iter()
                .next()
                .expect("Failed to get supported composite alpha");

            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(&engine_window.surface, Default::default())
                    .expect("Impossible to get surface formats")[0]
                    .0,
            );

            let window = engine_window
                .surface
                .object()
                .unwrap()
                .downcast_ref::<Window>()
                .expect("Impossible to get the window");
            let image_extent: [u32; 2] = window.inner_size().into();

            Swapchain::new(
                device.clone(),
                engine_window.surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: caps.min_image_count,
                    image_format,
                    image_extent,
                    image_usage: usage,
                    composite_alpha: alpha,
                    ..Default::default()
                },
            )
            .expect("Impossible to create the swapchain")
        };

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        //TODO: Create a queue for each queue family
        (
            Self {
                instance,
                device,
                queue: queues.next().unwrap(),
                swapchain_related: SwapchainRelated { swapchain, images },
                command_buffer_allocator,
                viewport,
            },
            engine_window,
        )
    }

    pub fn start_main_loop(
        mut self,
        engine_window: EngineWindow,
        mut render_loop: fn(engine: &mut Engine),
    ) {
        let mut recreate_swapchain = false;
        let mut previous_frame_end =
            Some(Box::new(sync::now(self.device.clone())) as Box<dyn GpuFuture>);

        let render_pass = vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: self.swapchain_related.swapchain.image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap();

        let mut framebuffers = window_size_dependent_setup(
            &self.swapchain_related.images,
            render_pass.clone(),
            &mut self.viewport,
        );

        //Handle close
        let mut done = false;

        engine_window.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    if recreate_swapchain {
                        self.swapchain_related
                            .recreate_swapchain(&engine_window.surface);

                        framebuffers = window_size_dependent_setup(
                            &self.swapchain_related.images,
                            render_pass.clone(),
                            &mut self.viewport,
                        );
                        recreate_swapchain = false;
                    }

                    previous_frame_end.as_mut().unwrap().cleanup_finished();
                    render_loop(&mut self);

                    // do our render operations here
                }
                _ => {}
            }
        });
    }
}

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
