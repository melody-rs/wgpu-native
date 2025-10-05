use super::from_u64_bits;
use crate::utils::{make_slice, string_view_into_label, string_view_into_str};
use crate::{map_enum, map_enum_with_undefined, native};
use std::borrow::Cow;

map_enum_with_undefined!(
    map_primitive_topology,
    WGPUPrimitiveTopology,
    wgt::PrimitiveTopology,
    "Unknown primitive topology",
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip
);

map_enum!(
    map_index_format,
    WGPUIndexFormat,
    wgt::IndexFormat,
    Uint16,
    Uint32
);

map_enum_with_undefined!(
    map_blend_factor,
    WGPUBlendFactor,
    wgt::BlendFactor,
    "Unknown blend factor",
    Zero,
    One,
    Src,
    OneMinusSrc,
    SrcAlpha,
    OneMinusSrcAlpha,
    Dst,
    OneMinusDst,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    Constant,
    OneMinusConstant,
    Src1,
    OneMinusSrc1,
    Src1Alpha,
    OneMinusSrc1Alpha
);

map_enum_with_undefined!(
    map_blend_operation,
    WGPUBlendOperation,
    wgt::BlendOperation,
    "Unknown blend operation",
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max
);

map_enum_with_undefined!(
    map_stencil_operation,
    WGPUStencilOperation,
    wgt::StencilOperation,
    "Unknown stencil operation",
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap
);

map_enum!(
    map_vertex_format,
    WGPUVertexFormat,
    wgt::VertexFormat,
    Uint8x2,
    Uint8x4,
    Sint8x2,
    Sint8x4,
    Unorm8x2,
    Unorm8x4,
    Snorm8x2,
    Snorm8x4,
    Uint16x2,
    Uint16x4,
    Sint16x2,
    Sint16x4,
    Unorm16x2,
    Unorm16x4,
    Snorm16x2,
    Snorm16x4,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4,
    Unorm10_10_10_2
);

#[cfg(feature = "glsl")]
map_enum!(
    map_shader_stage,
    WGPUShaderStage,
    naga::ShaderStage,
    Vertex,
    Fragment,
    Compute
);

// FIXME where should this go?
map_enum_with_undefined!(
    map_compare_function,
    WGPUCompareFunction,
    wgt::CompareFunction,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always
);

#[inline]
pub fn map_blend_component(native: native::WGPUBlendComponent) -> wgt::BlendComponent {
    wgt::BlendComponent {
        src_factor: map_blend_factor(native.srcFactor).unwrap_or(wgt::BlendFactor::One),
        dst_factor: map_blend_factor(native.dstFactor).unwrap_or(wgt::BlendFactor::Zero),
        operation: map_blend_operation(native.operation).unwrap_or(wgt::BlendOperation::Add),
    }
}

#[inline]
pub fn map_stencil_face_state(
    value: native::WGPUStencilFaceState,
    mode: &str,
) -> wgt::StencilFaceState {
    wgt::StencilFaceState {
        compare: map_compare_function(value.compare)
            .unwrap_or_else(|_| panic!("invalid compare function for {mode} stencil face state"))
            .unwrap_or(wgt::CompareFunction::Always),
        fail_op: map_stencil_operation(value.failOp).unwrap_or(wgt::StencilOperation::Keep),
        depth_fail_op: map_stencil_operation(value.depthFailOp)
            .unwrap_or(wgt::StencilOperation::Keep),
        pass_op: map_stencil_operation(value.passOp).unwrap_or(wgt::StencilOperation::Keep),
    }
}

fn map_polygon_mode(mode: native::WGPUPolygonMode) -> wgt::PolygonMode {
    match mode {
        native::WGPUPolygonMode_Fill => wgt::PolygonMode::Fill,
        native::WGPUPolygonMode_Line => wgt::PolygonMode::Line,
        native::WGPUPolygonMode_Point => wgt::PolygonMode::Point,
        _ => panic!("unknown polygon mode {mode}"),
    }
}

pub fn map_primitive_state(
    primitive: native::WGPUPrimitiveState,
    extras: Option<&native::WGPUPrimitiveStateExtras>,
) -> wgt::PrimitiveState {
    let polygon_mode = extras
        .map(|extras| map_polygon_mode(extras.polygonMode))
        .unwrap_or_default();
    let conservative = extras
        .map(|extras| extras.conservative != 0)
        .unwrap_or_default();

    wgt::PrimitiveState {
        topology: map_primitive_topology(primitive.topology)
            .unwrap_or(wgt::PrimitiveTopology::TriangleList),
        strip_index_format: map_index_format(primitive.stripIndexFormat).ok(),
        front_face: match primitive.frontFace {
            native::WGPUFrontFace_CCW | native::WGPUFrontFace_Undefined => wgt::FrontFace::Ccw,
            native::WGPUFrontFace_CW => wgt::FrontFace::Cw,
            _ => panic!("invalid front face for primitive state"),
        },
        cull_mode: match primitive.cullMode {
            native::WGPUCullMode_None | native::WGPUCullMode_Undefined => None,
            native::WGPUCullMode_Front => Some(wgt::Face::Front),
            native::WGPUCullMode_Back => Some(wgt::Face::Back),
            _ => panic!("invalid cull mode for primitive state"),
        },
        unclipped_depth: primitive.unclippedDepth != 0,
        polygon_mode,
        conservative,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ShaderParseError {
    #[cfg(feature = "spirv")]
    #[error(transparent)]
    Spirv(#[from] naga::front::spv::Error),
    #[cfg(feature = "glsl")]
    #[error(transparent)]
    Glsl(#[from] naga::front::glsl::ParseErrors),
}

#[inline]
pub unsafe fn map_shader_module<'a>(
    _: &native::WGPUShaderModuleDescriptor,
    spirv: Option<&native::WGPUShaderSourceSPIRV>,
    wgsl: Option<&native::WGPUShaderSourceWGSL>,
    glsl: Option<&native::WGPUShaderSourceGLSL>,
) -> Result<wgc::pipeline::ShaderModuleSource<'a>, ShaderParseError> {
    #[cfg(feature = "wgsl")]
    if let Some(wgsl) = wgsl {
        let str_slice: &str = string_view_into_str(wgsl.code).unwrap_or("");
        return Ok(wgc::pipeline::ShaderModuleSource::Wgsl(Cow::Borrowed(
            str_slice,
        )));
    }

    #[cfg(feature = "spirv")]
    if let Some(spirv) = spirv {
        let slice = make_slice(spirv.code, spirv.codeSize as usize);
        // Parse the given shader code and store its representation.
        let options = naga::front::spv::Options {
            adjust_coordinate_space: false, // we require NDC_Y_UP feature
            strict_capabilities: true,
            block_ctx_dump_prefix: None,
        };
        let frontend = naga::front::spv::Frontend::new(slice.iter().cloned(), &options);
        match frontend.parse() {
            Ok(module) => return Ok(wgc::pipeline::ShaderModuleSource::Naga(Cow::Owned(module))),
            Err(cause) => return Err(ShaderParseError::Spirv(cause)),
        };
    }

    #[cfg(feature = "glsl")]
    if let Some(glsl) = glsl {
        let str_slice: &str = string_view_into_str(glsl.code).unwrap_or("");
        let mut options = naga::front::glsl::Options::from(
            map_shader_stage(glsl.stage)
                .expect("invalid shader stage for shader module glsl descriptor"),
        );

        let raw_defines = make_slice(glsl.defines, glsl.defineCount as usize);
        for define in raw_defines {
            let name_str_slice: &str = string_view_into_str(define.name).unwrap_or("");
            let value_str_slice: &str = string_view_into_str(define.value).unwrap_or("");

            options
                .defines
                .insert(String::from(name_str_slice), String::from(value_str_slice));
        }

        let mut frontend = naga::front::glsl::Frontend::default();
        match frontend.parse(&options, str_slice) {
            Ok(module) => return Ok(wgc::pipeline::ShaderModuleSource::Naga(Cow::Owned(module))),
            Err(causes) => return Err(ShaderParseError::Glsl(causes)),
        };
    }

    panic!("Shader not provided.");
}

#[inline]
pub unsafe fn map_pipeline_layout_descriptor<'a>(
    des: &native::WGPUPipelineLayoutDescriptor,
    extras: Option<&native::WGPUPipelineLayoutExtras>,
) -> wgc::binding_model::PipelineLayoutDescriptor<'a> {
    let bind_group_layouts = make_slice(des.bindGroupLayouts, des.bindGroupLayoutCount)
        .iter()
        .map(|layout| {
            layout
                .as_ref()
                .expect("invalid bind group layout for pipeline layout descriptor")
                .id
        })
        .collect::<Vec<_>>();

    let push_constant_ranges = extras.map_or(Vec::new(), |extras| {
        make_slice(extras.pushConstantRanges, extras.pushConstantRangeCount)
            .iter()
            .map(|range| wgt::PushConstantRange {
                stages: from_u64_bits(range.stages)
                    .expect("invalid shader stage for push constant range"),
                range: range.start..range.end,
            })
            .collect()
    });

    wgc::binding_model::PipelineLayoutDescriptor {
        label: string_view_into_label(des.label),
        bind_group_layouts: Cow::from(bind_group_layouts),
        push_constant_ranges: Cow::from(push_constant_ranges),
    }
}
