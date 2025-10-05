use crate::map_enum;
use crate::native;
use crate::utils::string_view_into_str;

map_enum!(
    map_dxc_max_shader_model,
    WGPUDxcMaxShaderModel,
    wgt::DxcShaderModel,
    "Unknown shader model version",
    V6_0,
    V6_1,
    V6_2,
    V6_3,
    V6_4,
    V6_5,
    V6_6,
    V6_7
);

map_enum!(
    map_gl_fence_behavior,
    WGPUGLFenceBehaviour,
    wgt::GlFenceBehavior,
    "Unknown gl fence behavior",
    Normal,
    AutoFinish
);

map_enum!(
    map_gles3_minor_version,
    WGPUGles3MinorVersion,
    wgt::Gles3MinorVersion,
    "Unknown gles3 minor version",
    Automatic,
    Version0,
    Version1,
    Version2
);

#[inline]
pub fn map_instance_backend_flags(flags: native::WGPUInstanceBackend) -> wgt::Backends {
    if flags == native::WGPUInstanceBackend_All {
        return wgt::Backends::all();
    }

    let mut result = wgt::Backends::empty();
    if (flags & native::WGPUInstanceBackend_BrowserWebGPU) != 0 {
        result |= wgt::Backends::BROWSER_WEBGPU;
    }
    if (flags & native::WGPUInstanceBackend_Vulkan) != 0 {
        result |= wgt::Backends::VULKAN;
    }
    if (flags & native::WGPUInstanceBackend_GL) != 0 {
        result |= wgt::Backends::GL;
    }
    if (flags & native::WGPUInstanceBackend_Metal) != 0 {
        result |= wgt::Backends::METAL;
    }
    if (flags & native::WGPUInstanceBackend_DX12) != 0 {
        result |= wgt::Backends::DX12;
    }
    result
}

#[inline]
pub fn map_instance_flags(flags: native::WGPUInstanceFlag) -> wgt::InstanceFlags {
    let mut result = wgt::InstanceFlags::empty();
    if (flags & native::WGPUInstanceFlag_Debug) != 0 {
        result.insert(wgt::InstanceFlags::DEBUG);
    }
    if (flags & native::WGPUInstanceFlag_Validation) != 0 {
        result.insert(wgt::InstanceFlags::VALIDATION);
    }
    if (flags & native::WGPUInstanceFlag_DiscardHalLabels) != 0 {
        result.insert(wgt::InstanceFlags::DISCARD_HAL_LABELS);
    }
    result
}

#[inline]
pub unsafe fn map_instance_descriptor(
    _base: &native::WGPUInstanceDescriptor,
    extras: Option<&native::WGPUInstanceExtras>,
) -> wgt::InstanceDescriptor {
    if let Some(extras) = extras {
        let dx12_shader_compiler = match extras.dx12ShaderCompiler {
            native::WGPUDx12Compiler_Fxc => wgt::Dx12Compiler::Fxc,
            native::WGPUDx12Compiler_Dxc => match string_view_into_str(extras.dxcPath) {
                Some(dxc_path) => wgt::Dx12Compiler::DynamicDxc {
                    dxc_path: dxc_path.to_string(),
                    max_shader_model: map_dxc_max_shader_model(extras.dxcMaxShaderModel),
                },
                _ => wgt::Dx12Compiler::StaticDxc,
            },
            _ => wgt::Dx12Compiler::default(),
        };

        let for_resource_creation = unsafe { extras.budgetForDeviceCreation.as_ref() }.copied();
        let for_device_loss = unsafe { extras.budgetForDeviceCreation.as_ref() }.copied();

        wgt::InstanceDescriptor {
            backends: map_instance_backend_flags(extras.backends as native::WGPUInstanceBackend),
            backend_options: wgt::BackendOptions {
                gl: wgt::GlBackendOptions {
                    gles_minor_version: map_gles3_minor_version(extras.gles3MinorVersion),
                    fence_behavior: map_gl_fence_behavior(extras.glFenceBehaviour),
                },
                dx12: wgt::Dx12BackendOptions {
                    shader_compiler: dx12_shader_compiler,
                    ..Default::default()
                },
                noop: Default::default(),
            },
            flags: match extras.flags {
                native::WGPUInstanceFlag_Default => wgt::InstanceFlags::default(),
                flags => map_instance_flags(flags),
            },
            memory_budget_thresholds: wgt::MemoryBudgetThresholds {
                for_device_loss,
                for_resource_creation,
            },
        }
    } else {
        wgt::InstanceDescriptor::default()
    }
}
