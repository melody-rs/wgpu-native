use crate::native;

pub fn map_backend_type(backend: wgt::Backend) -> native::WGPUBackendType {
    match backend {
        wgt::Backend::Noop => native::WGPUBackendType_Null,
        wgt::Backend::Vulkan => native::WGPUBackendType_Vulkan,
        wgt::Backend::Metal => native::WGPUBackendType_Metal,
        wgt::Backend::Dx12 => native::WGPUBackendType_D3D12,
        wgt::Backend::Gl => native::WGPUBackendType_OpenGL,
        wgt::Backend::BrowserWebGpu => native::WGPUBackendType_WebGPU,
    }
}

pub fn map_adapter_type(device_type: wgt::DeviceType) -> native::WGPUAdapterType {
    match device_type {
        wgt::DeviceType::Other => native::WGPUAdapterType_Unknown,
        wgt::DeviceType::IntegratedGpu => native::WGPUAdapterType_IntegratedGPU,
        wgt::DeviceType::DiscreteGpu => native::WGPUAdapterType_DiscreteGPU,
        wgt::DeviceType::VirtualGpu => native::WGPUAdapterType_CPU, // close enough?
        wgt::DeviceType::Cpu => native::WGPUAdapterType_CPU,
    }
}
