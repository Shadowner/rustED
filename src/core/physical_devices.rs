use std::sync::Arc;

use vulkano::{
    device::{
        physical::{self, PhysicalDevice},
        DeviceExtensions,
    },
    instance::Instance,
    swapchain::Surface,
};

pub fn get_compatible_physical_devices(
    instance: &Arc<Instance>,
    surface_info: &Surface,
    device_requirement: &DeviceExtensions,
) -> Vec<(Arc<PhysicalDevice>, u32)> {
    let physical_devices = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_requirement))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, info)| {
                    info.queue_flags.graphics
                        && p.surface_support(i as u32, &surface_info).unwrap_or(false)
                })
                .map(|f| (p, f as u32))
        })
        .collect::<Vec<(Arc<PhysicalDevice>, u32)>>();

    return physical_devices;
}
pub fn get_prefered_physical_device(
    physical_devices: Vec<(Arc<PhysicalDevice>, u32)>,
) -> (Arc<PhysicalDevice>, u32) {
    physical_devices
        .iter()
        .min_by_key(|(device, _)| match device.properties().device_type {
            physical::PhysicalDeviceType::DiscreteGpu => 0,
            physical::PhysicalDeviceType::IntegratedGpu => 1,
            physical::PhysicalDeviceType::VirtualGpu => 2,
            physical::PhysicalDeviceType::Cpu => 3,
            physical::PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("No compatible physical device found");

    return physical_devices.first().unwrap().clone();
}
