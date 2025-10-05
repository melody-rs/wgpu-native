use crate::native;

// These are defined as UINT64_MAX in the header, but bindgen currently can't process that define.
// See https://github.com/rust-lang/rust-bindgen/issues/2822
pub const WGPU_LIMIT_U64_UNDEFINED: u64 = u64::MAX;

#[inline]
pub fn map_required_limits(
    limits: &native::WGPULimits,
    base_limits: wgt::Limits,
    extras: Option<&native::WGPUNativeLimits>,
) -> wgt::Limits {
    let mut wgt_limits = base_limits;
    if limits.maxTextureDimension1D != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_dimension_1d = limits.maxTextureDimension1D;
    }
    if limits.maxTextureDimension2D != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_dimension_2d = limits.maxTextureDimension2D;
    }
    if limits.maxTextureDimension3D != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_dimension_3d = limits.maxTextureDimension3D;
    }
    if limits.maxTextureArrayLayers != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_texture_array_layers = limits.maxTextureArrayLayers;
    }
    if limits.maxBindGroups != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_bind_groups = limits.maxBindGroups;
    }
    // TODO: not yet in wgt
    // if limits.maxBindGroupsPlusVertexBuffers != native::WGPU_LIMIT_U32_UNDEFINED {
    //     wgt_limits.max_bind_groups_plus_vertex_buffers = limits.maxBindGroupsPlusVertexBuffers;
    // }
    if limits.maxBindingsPerBindGroup != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_bindings_per_bind_group = limits.maxBindingsPerBindGroup;
    }
    if limits.maxDynamicUniformBuffersPerPipelineLayout != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_dynamic_uniform_buffers_per_pipeline_layout =
            limits.maxDynamicUniformBuffersPerPipelineLayout;
    }
    if limits.maxDynamicStorageBuffersPerPipelineLayout != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_dynamic_storage_buffers_per_pipeline_layout =
            limits.maxDynamicStorageBuffersPerPipelineLayout;
    }
    if limits.maxSampledTexturesPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_sampled_textures_per_shader_stage = limits.maxSampledTexturesPerShaderStage;
    }
    if limits.maxSamplersPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_samplers_per_shader_stage = limits.maxSamplersPerShaderStage;
    }
    if limits.maxStorageBuffersPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_storage_buffers_per_shader_stage = limits.maxStorageBuffersPerShaderStage;
    }
    if limits.maxStorageTexturesPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_storage_textures_per_shader_stage = limits.maxStorageTexturesPerShaderStage;
    }
    if limits.maxUniformBuffersPerShaderStage != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_uniform_buffers_per_shader_stage = limits.maxUniformBuffersPerShaderStage;
    }
    if limits.maxUniformBufferBindingSize != WGPU_LIMIT_U64_UNDEFINED {
        wgt_limits.max_uniform_buffer_binding_size = limits.maxUniformBufferBindingSize as u32;
    }
    if limits.maxStorageBufferBindingSize != WGPU_LIMIT_U64_UNDEFINED {
        wgt_limits.max_storage_buffer_binding_size = limits.maxStorageBufferBindingSize as u32;
    }
    if limits.minUniformBufferOffsetAlignment != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.min_uniform_buffer_offset_alignment = limits.minUniformBufferOffsetAlignment;
    }
    if limits.minStorageBufferOffsetAlignment != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.min_storage_buffer_offset_alignment = limits.minStorageBufferOffsetAlignment;
    }
    if limits.maxVertexBuffers != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_vertex_buffers = limits.maxVertexBuffers;
    }
    if limits.maxBufferSize != WGPU_LIMIT_U64_UNDEFINED {
        wgt_limits.max_buffer_size = limits.maxBufferSize;
    }
    if limits.maxVertexAttributes != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_vertex_attributes = limits.maxVertexAttributes;
    }
    if limits.maxVertexBufferArrayStride != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_vertex_buffer_array_stride = limits.maxVertexBufferArrayStride;
    }
    // TODO: not yet in wgt
    // if limits.maxInterStageShaderVariables != native::WGPU_LIMIT_U32_UNDEFINED {
    //     wgt_limits.max_inter_stage_shader_variables = limits.maxInterStageShaderVariables;
    // }
    if limits.maxColorAttachments != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_color_attachments = limits.maxColorAttachments;
    }
    if limits.maxColorAttachmentBytesPerSample != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_color_attachment_bytes_per_sample = limits.maxColorAttachmentBytesPerSample;
    }
    if limits.maxComputeWorkgroupStorageSize != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_storage_size = limits.maxComputeWorkgroupStorageSize;
    }
    if limits.maxComputeInvocationsPerWorkgroup != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_invocations_per_workgroup = limits.maxComputeInvocationsPerWorkgroup;
    }
    if limits.maxComputeWorkgroupSizeX != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_size_x = limits.maxComputeWorkgroupSizeX;
    }
    if limits.maxComputeWorkgroupSizeY != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_size_y = limits.maxComputeWorkgroupSizeY;
    }
    if limits.maxComputeWorkgroupSizeZ != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroup_size_z = limits.maxComputeWorkgroupSizeZ;
    }
    if limits.maxComputeWorkgroupsPerDimension != native::WGPU_LIMIT_U32_UNDEFINED {
        wgt_limits.max_compute_workgroups_per_dimension = limits.maxComputeWorkgroupsPerDimension;
    }
    if let Some(limits) = extras {
        if limits.maxPushConstantSize != native::WGPU_LIMIT_U32_UNDEFINED {
            wgt_limits.max_push_constant_size = limits.maxPushConstantSize;
        }
        if limits.maxNonSamplerBindings != native::WGPU_LIMIT_U32_UNDEFINED {
            wgt_limits.max_non_sampler_bindings = limits.maxNonSamplerBindings;
        }
    }
    wgt_limits
}

#[inline]
pub fn write_limits_struct(wgt_limits: wgt::Limits, limits: &mut native::WGPULimits) {
    limits.maxTextureDimension1D = wgt_limits.max_texture_dimension_1d;
    limits.maxTextureDimension2D = wgt_limits.max_texture_dimension_2d;
    limits.maxTextureDimension3D = wgt_limits.max_texture_dimension_3d;
    limits.maxTextureArrayLayers = wgt_limits.max_texture_array_layers;
    limits.maxBindGroups = wgt_limits.max_bind_groups;
    // TODO: not yet in wgt
    // limits.maxBindGroupsPlusVertexBuffers = wgt_limits.max_bind_groups_plus_vertex_buffers;
    limits.maxBindingsPerBindGroup = wgt_limits.max_bindings_per_bind_group;
    limits.maxDynamicUniformBuffersPerPipelineLayout =
        wgt_limits.max_dynamic_uniform_buffers_per_pipeline_layout;
    limits.maxDynamicStorageBuffersPerPipelineLayout =
        wgt_limits.max_dynamic_storage_buffers_per_pipeline_layout;
    limits.maxSampledTexturesPerShaderStage = wgt_limits.max_sampled_textures_per_shader_stage;
    limits.maxSamplersPerShaderStage = wgt_limits.max_samplers_per_shader_stage;
    limits.maxStorageBuffersPerShaderStage = wgt_limits.max_storage_buffers_per_shader_stage;
    limits.maxStorageTexturesPerShaderStage = wgt_limits.max_storage_textures_per_shader_stage;
    limits.maxUniformBuffersPerShaderStage = wgt_limits.max_uniform_buffers_per_shader_stage;
    limits.maxUniformBufferBindingSize = wgt_limits.max_uniform_buffer_binding_size as _;
    limits.maxStorageBufferBindingSize = wgt_limits.max_storage_buffer_binding_size as _;
    limits.maxVertexBuffers = wgt_limits.max_vertex_buffers;
    limits.maxBufferSize = wgt_limits.max_buffer_size;
    limits.maxVertexAttributes = wgt_limits.max_vertex_attributes;
    limits.maxVertexBufferArrayStride = wgt_limits.max_vertex_buffer_array_stride;
    limits.minUniformBufferOffsetAlignment = wgt_limits.min_uniform_buffer_offset_alignment;
    limits.minStorageBufferOffsetAlignment = wgt_limits.min_storage_buffer_offset_alignment;
    // TODO: not yet in wgt
    // limits.maxInterStageShaderVariables = wgt_limits.max_inter_stage_shader_variables;
    limits.maxColorAttachments = wgt_limits.max_color_attachments;
    limits.maxColorAttachmentBytesPerSample = wgt_limits.max_color_attachment_bytes_per_sample;
    limits.maxComputeWorkgroupStorageSize = wgt_limits.max_compute_workgroup_storage_size;
    limits.maxComputeInvocationsPerWorkgroup = wgt_limits.max_compute_invocations_per_workgroup;
    limits.maxComputeWorkgroupSizeX = wgt_limits.max_compute_workgroup_size_x;
    limits.maxComputeWorkgroupSizeY = wgt_limits.max_compute_workgroup_size_y;
    limits.maxComputeWorkgroupSizeZ = wgt_limits.max_compute_workgroup_size_z;
    limits.maxComputeWorkgroupsPerDimension = wgt_limits.max_compute_workgroups_per_dimension;

    if let Some(native::WGPUChainedStructOut {
        sType: native::WGPUSType_NativeLimits,
        ..
    }) = unsafe { limits.nextInChain.as_ref() }
    {
        let native_limits = limits.nextInChain.cast::<native::WGPUNativeLimits>();
        unsafe {
            (*native_limits).maxPushConstantSize = wgt_limits.max_push_constant_size;
            (*native_limits).maxNonSamplerBindings = wgt_limits.max_non_sampler_bindings;
        }
    };
}
