use crate::native;

#[inline]
pub fn map_features(features: &[native::WGPUFeatureName]) -> wgt::Features {
    let mut temp = wgt::Features::empty();

    features.iter().for_each(|f| {
        if let Some(feature) = map_feature(*f) {
            temp.insert(feature);
        };
    });

    temp
}

#[inline]
#[rustfmt::skip]
pub fn map_feature(feature: native::WGPUFeatureName) -> Option<wgt::Features> {
    use wgt::Features;

    match feature {
        native::WGPUFeatureName_DepthClipControl => Some(Features::DEPTH_CLIP_CONTROL),
        native::WGPUFeatureName_Depth32FloatStencil8 => Some(Features::DEPTH32FLOAT_STENCIL8),
        native::WGPUFeatureName_TextureCompressionBC => Some(Features::TEXTURE_COMPRESSION_BC),
        // TODO: WGPUFeatureName_TextureCompressionBCSliced3D
        native::WGPUFeatureName_TextureCompressionETC2 => Some(Features::TEXTURE_COMPRESSION_ETC2),
        native::WGPUFeatureName_TextureCompressionASTC => Some(Features::TEXTURE_COMPRESSION_ASTC),
        // TODO: WGPUFeatureName_TextureCompressionASTCSliced3D
        native::WGPUFeatureName_TimestampQuery => Some(Features::TIMESTAMP_QUERY),
        native::WGPUFeatureName_IndirectFirstInstance => Some(Features::INDIRECT_FIRST_INSTANCE),
        native::WGPUFeatureName_ShaderF16 => Some(Features::SHADER_F16),
        native::WGPUFeatureName_RG11B10UfloatRenderable => Some(Features::RG11B10UFLOAT_RENDERABLE),
        native::WGPUFeatureName_BGRA8UnormStorage => Some(Features::BGRA8UNORM_STORAGE),
        // TODO: WGPUFeatureName_ClipDistances
        // TODO: WGPUFeatureName_Float32Blendable
        native::WGPUFeatureName_Float32Filterable => Some(Features::FLOAT32_FILTERABLE),
        native::WGPUFeatureName_DualSourceBlending => Some(Features::DUAL_SOURCE_BLENDING),

        // wgpu-rs only features
        native::WGPUNativeFeature_PushConstants => Some(Features::PUSH_CONSTANTS),
        native::WGPUNativeFeature_TextureAdapterSpecificFormatFeatures => Some(Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES),
        native::WGPUNativeFeature_MultiDrawIndirectCount => Some(Features::MULTI_DRAW_INDIRECT_COUNT),
        native::WGPUNativeFeature_VertexWritableStorage => Some(Features::VERTEX_WRITABLE_STORAGE),
        native::WGPUNativeFeature_TextureBindingArray => Some(Features::TEXTURE_BINDING_ARRAY),
        native::WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing => Some(Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
        native::WGPUNativeFeature_PipelineStatisticsQuery => Some(Features::PIPELINE_STATISTICS_QUERY),
        native::WGPUNativeFeature_StorageResourceBindingArray => Some(Features::STORAGE_RESOURCE_BINDING_ARRAY),
        native::WGPUNativeFeature_PartiallyBoundBindingArray => Some(Features::PARTIALLY_BOUND_BINDING_ARRAY),
        native::WGPUNativeFeature_TextureFormat16bitNorm => Some(Features::TEXTURE_FORMAT_16BIT_NORM),
        native::WGPUNativeFeature_TextureCompressionAstcHdr => Some(Features::TEXTURE_COMPRESSION_ASTC_HDR),
        native::WGPUNativeFeature_TimestampQueryInsidePasses => Some(Features::TIMESTAMP_QUERY_INSIDE_PASSES),
        native::WGPUNativeFeature_TimestampQueryInsideEncoders => Some(Features::TIMESTAMP_QUERY_INSIDE_ENCODERS),
        native::WGPUNativeFeature_MappablePrimaryBuffers => Some(Features::MAPPABLE_PRIMARY_BUFFERS),
        native::WGPUNativeFeature_BufferBindingArray => Some(Features::BUFFER_BINDING_ARRAY),
        // TODO: fix this, UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING is not supported anymore https://github.com/gfx-rs/wgpu/issues/4407
        // native::WGPUNativeFeature_UniformBufferAndStorageTextureArrayNonUniformIndexing => Some(Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING),
        // TODO: requires wgpu.h api change
        // native::WGPUNativeFeature_AddressModeClampToZero => Some(Features::ADDRESS_MODE_CLAMP_TO_ZERO),
        // native::WGPUNativeFeature_AddressModeClampToBorder => Some(Features::ADDRESS_MODE_CLAMP_TO_BORDER),
        native::WGPUNativeFeature_PolygonModeLine => Some(Features::POLYGON_MODE_LINE),
        native::WGPUNativeFeature_PolygonModePoint => Some(Features::POLYGON_MODE_POINT),
        native::WGPUNativeFeature_ConservativeRasterization => Some(Features::CONSERVATIVE_RASTERIZATION),
        // native::WGPUNativeFeature_ClearTexture => Some(Features::CLEAR_TEXTURE),
        // native::WGPUNativeFeature_Multiview => Some(Features::MULTIVIEW),
        native::WGPUNativeFeature_VertexAttribute64bit => Some(Features::VERTEX_ATTRIBUTE_64BIT),
        native::WGPUNativeFeature_TextureFormatNv12 => Some(Features::TEXTURE_FORMAT_NV12),
        native::WGPUNativeFeature_RayQuery => Some(Features::EXPERIMENTAL_RAY_QUERY),
        native::WGPUNativeFeature_ShaderF64 => Some(Features::SHADER_F64),
        native::WGPUNativeFeature_ShaderInt64 => Some(Features::SHADER_INT64),
        native::WGPUNativeFeature_ShaderPrimitiveIndex => Some(Features::SHADER_PRIMITIVE_INDEX),
        native::WGPUNativeFeature_ShaderEarlyDepthTest => Some(Features::SHADER_EARLY_DEPTH_TEST),
        native::WGPUNativeFeature_Subgroup => Some(Features::SUBGROUP),
        native::WGPUNativeFeature_SubgroupVertex => Some(Features::SUBGROUP_VERTEX),
        native::WGPUNativeFeature_SubgroupBarrier => Some(Features::SUBGROUP_BARRIER),
        // fallback, probably not available in wgpu-core
        _ => None,
    }
}

#[inline]
pub fn features_to_native(features: wgt::Features) -> Vec<native::WGPUFeatureName> {
    let mut temp = Vec::new();

    if features.contains(wgt::Features::DEPTH_CLIP_CONTROL) {
        temp.push(native::WGPUFeatureName_DepthClipControl);
    }
    if features.contains(wgt::Features::DEPTH32FLOAT_STENCIL8) {
        temp.push(native::WGPUFeatureName_Depth32FloatStencil8);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_BC) {
        temp.push(native::WGPUFeatureName_TextureCompressionBC);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ETC2) {
        temp.push(native::WGPUFeatureName_TextureCompressionETC2);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ASTC) {
        temp.push(native::WGPUFeatureName_TextureCompressionASTC);
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY) {
        temp.push(native::WGPUFeatureName_TimestampQuery);
    }
    if features.contains(wgt::Features::INDIRECT_FIRST_INSTANCE) {
        temp.push(native::WGPUFeatureName_IndirectFirstInstance);
    }
    if features.contains(wgt::Features::SHADER_F16) {
        temp.push(native::WGPUFeatureName_ShaderF16);
    }
    if features.contains(wgt::Features::RG11B10UFLOAT_RENDERABLE) {
        temp.push(native::WGPUFeatureName_RG11B10UfloatRenderable);
    }
    if features.contains(wgt::Features::BGRA8UNORM_STORAGE) {
        temp.push(native::WGPUFeatureName_BGRA8UnormStorage);
    }
    if features.contains(wgt::Features::FLOAT32_FILTERABLE) {
        temp.push(native::WGPUFeatureName_Float32Filterable);
    }
    if features.contains(wgt::Features::DUAL_SOURCE_BLENDING) {
        temp.push(native::WGPUFeatureName_DualSourceBlending);
    }
    // wgpu-rs only features
    if features.contains(wgt::Features::PUSH_CONSTANTS) {
        temp.push(native::WGPUNativeFeature_PushConstants);
    }
    if features.contains(wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES) {
        temp.push(native::WGPUNativeFeature_TextureAdapterSpecificFormatFeatures);
    }
    if features.contains(wgt::Features::MULTI_DRAW_INDIRECT_COUNT) {
        temp.push(native::WGPUNativeFeature_MultiDrawIndirectCount);
    }
    if features.contains(wgt::Features::VERTEX_WRITABLE_STORAGE) {
        temp.push(native::WGPUNativeFeature_VertexWritableStorage);
    }
    if features.contains(wgt::Features::TEXTURE_BINDING_ARRAY) {
        temp.push(native::WGPUNativeFeature_TextureBindingArray);
    }
    if features
        .contains(wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
    {
        temp.push(native::WGPUNativeFeature_SampledTextureAndStorageBufferArrayNonUniformIndexing);
    }
    if features.contains(wgt::Features::PIPELINE_STATISTICS_QUERY) {
        temp.push(native::WGPUNativeFeature_PipelineStatisticsQuery);
    }
    if features.contains(wgt::Features::STORAGE_RESOURCE_BINDING_ARRAY) {
        temp.push(native::WGPUNativeFeature_StorageResourceBindingArray);
    }
    if features.contains(wgt::Features::PARTIALLY_BOUND_BINDING_ARRAY) {
        temp.push(native::WGPUNativeFeature_PartiallyBoundBindingArray);
    }
    if features.contains(wgt::Features::TEXTURE_FORMAT_16BIT_NORM) {
        temp.push(native::WGPUNativeFeature_TextureFormat16bitNorm);
    }
    if features.contains(wgt::Features::TEXTURE_COMPRESSION_ASTC_HDR) {
        temp.push(native::WGPUNativeFeature_TextureCompressionAstcHdr);
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY_INSIDE_PASSES) {
        temp.push(native::WGPUNativeFeature_TimestampQueryInsidePasses);
    }
    if features.contains(wgt::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS) {
        temp.push(native::WGPUNativeFeature_TimestampQueryInsideEncoders);
    }
    if features.contains(wgt::Features::MAPPABLE_PRIMARY_BUFFERS) {
        temp.push(native::WGPUNativeFeature_MappablePrimaryBuffers);
    }
    if features.contains(wgt::Features::BUFFER_BINDING_ARRAY) {
        temp.push(native::WGPUNativeFeature_BufferBindingArray);
    }
    // TODO: fix this, UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING is not supported anymore https://github.com/gfx-rs/wgpu/issues/4407
    // if features
    //     .contains(wgt::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING)
    // {
    //     temp.push(native::WGPUNativeFeature_UniformBufferAndStorageTextureArrayNonUniformIndexing);
    // }
    // TODO: requires wgpu.h api change
    // if features.contains(wgt::Features::ADDRESS_MODE_CLAMP_TO_ZERO) {
    //     temp.push(native::WGPUNativeFeature_AddressModeClampToZero);
    // }
    // if features.contains(wgt::Features::ADDRESS_MODE_CLAMP_TO_BORDER) {
    //     temp.push(native::WGPUNativeFeature_AddressModeClampToBorder);
    // }
    if features.contains(wgt::Features::POLYGON_MODE_LINE) {
        temp.push(native::WGPUNativeFeature_PolygonModeLine);
    }
    if features.contains(wgt::Features::POLYGON_MODE_POINT) {
        temp.push(native::WGPUNativeFeature_PolygonModePoint);
    }
    if features.contains(wgt::Features::CONSERVATIVE_RASTERIZATION) {
        temp.push(native::WGPUNativeFeature_ConservativeRasterization);
    }
    // if features.contains(wgt::Features::CLEAR_TEXTURE) {
    //     temp.push(native::WGPUNativeFeature_ClearTexture);
    // }
    // if features.contains(wgt::Features::MULTIVIEW) {
    //     temp.push(native::WGPUNativeFeature_Multiview);
    // }
    if features.contains(wgt::Features::VERTEX_ATTRIBUTE_64BIT) {
        temp.push(native::WGPUNativeFeature_VertexAttribute64bit);
    }
    if features.contains(wgt::Features::TEXTURE_FORMAT_NV12) {
        temp.push(native::WGPUNativeFeature_TextureFormatNv12);
    }
    if features.contains(wgt::Features::EXPERIMENTAL_RAY_QUERY) {
        temp.push(native::WGPUNativeFeature_RayQuery);
    }
    if features.contains(wgt::Features::SHADER_F64) {
        temp.push(native::WGPUNativeFeature_ShaderF64);
    }
    if features.contains(wgt::Features::SHADER_INT64) {
        temp.push(native::WGPUNativeFeature_ShaderInt64);
    }
    if features.contains(wgt::Features::SHADER_I16) {
        temp.push(native::WGPUNativeFeature_ShaderI16);
    }
    if features.contains(wgt::Features::SHADER_PRIMITIVE_INDEX) {
        temp.push(native::WGPUNativeFeature_ShaderPrimitiveIndex);
    }
    if features.contains(wgt::Features::SHADER_EARLY_DEPTH_TEST) {
        temp.push(native::WGPUNativeFeature_ShaderEarlyDepthTest);
    }
    if features.contains(wgt::Features::SUBGROUP) {
        temp.push(native::WGPUNativeFeature_Subgroup);
    }
    if features.contains(wgt::Features::SUBGROUP_VERTEX) {
        temp.push(native::WGPUNativeFeature_SubgroupVertex);
    }
    if features.contains(wgt::Features::SUBGROUP_BARRIER) {
        temp.push(native::WGPUNativeFeature_SubgroupBarrier);
    }

    temp
}
