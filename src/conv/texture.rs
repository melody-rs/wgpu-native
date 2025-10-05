use crate::native;

#[inline]
pub fn map_texture_usage_flags(flags: native::WGPUTextureUsage) -> wgt::TextureUsages {
    let mut temp = wgt::TextureUsages::empty();
    if (flags & native::WGPUTextureUsage_CopySrc) != 0 {
        temp.insert(wgt::TextureUsages::COPY_SRC);
    }
    if (flags & native::WGPUTextureUsage_CopyDst) != 0 {
        temp.insert(wgt::TextureUsages::COPY_DST);
    }
    if (flags & native::WGPUTextureUsage_TextureBinding) != 0 {
        temp.insert(wgt::TextureUsages::TEXTURE_BINDING);
    }
    if (flags & native::WGPUTextureUsage_RenderAttachment) != 0 {
        temp.insert(wgt::TextureUsages::RENDER_ATTACHMENT);
    }
    temp
}

#[inline]
pub fn to_native_texture_usage_flags(flags: wgt::TextureUsages) -> native::WGPUTextureUsage {
    let mut flag = 0;
    if flags.contains(wgt::TextureUsages::COPY_SRC) {
        flag |= native::WGPUTextureUsage_CopySrc;
    }
    if flags.contains(wgt::TextureUsages::COPY_DST) {
        flag |= native::WGPUTextureUsage_CopySrc;
    }
    if flags.contains(wgt::TextureUsages::TEXTURE_BINDING) {
        flag |= native::WGPUTextureUsage_TextureBinding;
    }
    if flags.contains(wgt::TextureUsages::RENDER_ATTACHMENT) {
        flag |= native::WGPUTextureUsage_RenderAttachment;
    }
    flag
}

#[inline]
pub fn map_texture_view_dimension(
    value: native::WGPUTextureViewDimension,
) -> Option<wgt::TextureViewDimension> {
    // This doesn't use map_enum_with_undefined! because the enum name after the _
    // isn't a valid ident on its own for the macro.
    match value {
        native::WGPUTextureViewDimension_1D => Some(wgt::TextureViewDimension::D1),
        native::WGPUTextureViewDimension_2D => Some(wgt::TextureViewDimension::D2),
        native::WGPUTextureViewDimension_2DArray => Some(wgt::TextureViewDimension::D2Array),
        native::WGPUTextureViewDimension_Cube => Some(wgt::TextureViewDimension::Cube),
        native::WGPUTextureViewDimension_CubeArray => Some(wgt::TextureViewDimension::CubeArray),
        native::WGPUTextureViewDimension_3D => Some(wgt::TextureViewDimension::D3),
        native::WGPUTextureDimension_Undefined => None,
        _ => panic!("Unknown texture view dimension"),
    }
}

#[inline]
pub fn map_texture_dimension(value: native::WGPUTextureDimension) -> Option<wgt::TextureDimension> {
    // This doesn't use map_enum_with_undefined! because the enum name after the _
    // isn't a valid ident on its own for the macro.
    match value {
        native::WGPUTextureDimension_1D => Some(wgt::TextureDimension::D1),
        native::WGPUTextureDimension_2D => Some(wgt::TextureDimension::D2),
        native::WGPUTextureDimension_3D => Some(wgt::TextureDimension::D3),
        native::WGPUTextureDimension_Undefined => None,
        x => panic!("Unknown texture dimension: {x}"),
    }
}

#[rustfmt::skip]
#[inline]
pub fn map_texture_format(value: native::WGPUTextureFormat) -> Option<wgt::TextureFormat> {
    use wgt::{AstcBlock, AstcChannel};

    match value {
        native::WGPUTextureFormat_Undefined => None,
        native::WGPUTextureFormat_R8Unorm => Some(wgt::TextureFormat::R8Unorm),
        native::WGPUTextureFormat_R8Snorm => Some(wgt::TextureFormat::R8Snorm),
        native::WGPUTextureFormat_R8Uint => Some(wgt::TextureFormat::R8Uint),
        native::WGPUTextureFormat_R8Sint => Some(wgt::TextureFormat::R8Sint),
        native::WGPUTextureFormat_R16Uint => Some(wgt::TextureFormat::R16Uint),
        native::WGPUTextureFormat_R16Sint => Some(wgt::TextureFormat::R16Sint),
        native::WGPUTextureFormat_R16Float => Some(wgt::TextureFormat::R16Float),
        native::WGPUTextureFormat_RG8Unorm => Some(wgt::TextureFormat::Rg8Unorm),
        native::WGPUTextureFormat_RG8Snorm => Some(wgt::TextureFormat::Rg8Snorm),
        native::WGPUTextureFormat_RG8Uint => Some(wgt::TextureFormat::Rg8Uint),
        native::WGPUTextureFormat_RG8Sint => Some(wgt::TextureFormat::Rg8Sint),
        native::WGPUTextureFormat_R32Float => Some(wgt::TextureFormat::R32Float),
        native::WGPUTextureFormat_R32Uint => Some(wgt::TextureFormat::R32Uint),
        native::WGPUTextureFormat_R32Sint => Some(wgt::TextureFormat::R32Sint),
        native::WGPUTextureFormat_RG16Uint => Some(wgt::TextureFormat::Rg16Uint),
        native::WGPUTextureFormat_RG16Sint => Some(wgt::TextureFormat::Rg16Sint),
        native::WGPUTextureFormat_RG16Float => Some(wgt::TextureFormat::Rg16Float),
        native::WGPUTextureFormat_RGBA8Unorm => Some(wgt::TextureFormat::Rgba8Unorm),
        native::WGPUTextureFormat_RGBA8UnormSrgb => Some(wgt::TextureFormat::Rgba8UnormSrgb),
        native::WGPUTextureFormat_RGBA8Snorm => Some(wgt::TextureFormat::Rgba8Snorm),
        native::WGPUTextureFormat_RGBA8Uint => Some(wgt::TextureFormat::Rgba8Uint),
        native::WGPUTextureFormat_RGBA8Sint => Some(wgt::TextureFormat::Rgba8Sint),
        native::WGPUTextureFormat_BGRA8Unorm => Some(wgt::TextureFormat::Bgra8Unorm),
        native::WGPUTextureFormat_BGRA8UnormSrgb => Some(wgt::TextureFormat::Bgra8UnormSrgb),
        native::WGPUTextureFormat_RGB10A2Uint => Some(wgt::TextureFormat::Rgb10a2Uint),
        native::WGPUTextureFormat_RGB10A2Unorm => Some(wgt::TextureFormat::Rgb10a2Unorm),
        native::WGPUTextureFormat_RG11B10Ufloat => Some(wgt::TextureFormat::Rg11b10Ufloat),
        native::WGPUTextureFormat_RGB9E5Ufloat => Some(wgt::TextureFormat::Rgb9e5Ufloat),
        native::WGPUTextureFormat_RG32Float => Some(wgt::TextureFormat::Rg32Float),
        native::WGPUTextureFormat_RG32Uint => Some(wgt::TextureFormat::Rg32Uint),
        native::WGPUTextureFormat_RG32Sint => Some(wgt::TextureFormat::Rg32Sint),
        native::WGPUTextureFormat_RGBA16Uint => Some(wgt::TextureFormat::Rgba16Uint),
        native::WGPUTextureFormat_RGBA16Sint => Some(wgt::TextureFormat::Rgba16Sint),
        native::WGPUTextureFormat_RGBA16Float => Some(wgt::TextureFormat::Rgba16Float),
        native::WGPUTextureFormat_RGBA32Float => Some(wgt::TextureFormat::Rgba32Float),
        native::WGPUTextureFormat_RGBA32Uint => Some(wgt::TextureFormat::Rgba32Uint),
        native::WGPUTextureFormat_RGBA32Sint => Some(wgt::TextureFormat::Rgba32Sint),
        native::WGPUTextureFormat_Stencil8 => Some(wgt::TextureFormat::Stencil8),
        native::WGPUTextureFormat_Depth16Unorm => Some(wgt::TextureFormat::Depth16Unorm),
        native::WGPUTextureFormat_Depth24Plus => Some(wgt::TextureFormat::Depth24Plus),
        native::WGPUTextureFormat_Depth24PlusStencil8 => Some(wgt::TextureFormat::Depth24PlusStencil8),
        native::WGPUTextureFormat_Depth32Float => Some(wgt::TextureFormat::Depth32Float),
        native::WGPUTextureFormat_Depth32FloatStencil8 => Some(wgt::TextureFormat::Depth32FloatStencil8),
        native::WGPUTextureFormat_BC1RGBAUnorm => Some(wgt::TextureFormat::Bc1RgbaUnorm),
        native::WGPUTextureFormat_BC1RGBAUnormSrgb => Some(wgt::TextureFormat::Bc1RgbaUnormSrgb),
        native::WGPUTextureFormat_BC2RGBAUnorm => Some(wgt::TextureFormat::Bc2RgbaUnorm),
        native::WGPUTextureFormat_BC2RGBAUnormSrgb => Some(wgt::TextureFormat::Bc2RgbaUnormSrgb),
        native::WGPUTextureFormat_BC3RGBAUnorm => Some(wgt::TextureFormat::Bc3RgbaUnorm),
        native::WGPUTextureFormat_BC3RGBAUnormSrgb => Some(wgt::TextureFormat::Bc3RgbaUnormSrgb),
        native::WGPUTextureFormat_BC4RUnorm => Some(wgt::TextureFormat::Bc4RUnorm),
        native::WGPUTextureFormat_BC4RSnorm => Some(wgt::TextureFormat::Bc4RSnorm),
        native::WGPUTextureFormat_BC5RGUnorm => Some(wgt::TextureFormat::Bc5RgUnorm),
        native::WGPUTextureFormat_BC5RGSnorm => Some(wgt::TextureFormat::Bc5RgSnorm),
        native::WGPUTextureFormat_BC6HRGBUfloat => Some(wgt::TextureFormat::Bc6hRgbUfloat),
        native::WGPUTextureFormat_BC6HRGBFloat => Some(wgt::TextureFormat::Bc6hRgbFloat),
        native::WGPUTextureFormat_BC7RGBAUnorm => Some(wgt::TextureFormat::Bc7RgbaUnorm),
        native::WGPUTextureFormat_BC7RGBAUnormSrgb => Some(wgt::TextureFormat::Bc7RgbaUnormSrgb),
        native::WGPUTextureFormat_ETC2RGB8Unorm => Some(wgt::TextureFormat::Etc2Rgb8Unorm),
        native::WGPUTextureFormat_ETC2RGB8UnormSrgb => Some(wgt::TextureFormat::Etc2Rgb8UnormSrgb),
        native::WGPUTextureFormat_ETC2RGB8A1Unorm => Some(wgt::TextureFormat::Etc2Rgb8A1Unorm),
        native::WGPUTextureFormat_ETC2RGB8A1UnormSrgb => Some(wgt::TextureFormat::Etc2Rgb8A1UnormSrgb),
        native::WGPUTextureFormat_ETC2RGBA8Unorm => Some(wgt::TextureFormat::Etc2Rgba8Unorm),
        native::WGPUTextureFormat_ETC2RGBA8UnormSrgb => Some(wgt::TextureFormat::Etc2Rgba8UnormSrgb),
        native::WGPUTextureFormat_EACR11Unorm => Some(wgt::TextureFormat::EacR11Unorm),
        native::WGPUTextureFormat_EACR11Snorm => Some(wgt::TextureFormat::EacR11Snorm),
        native::WGPUTextureFormat_EACRG11Unorm => Some(wgt::TextureFormat::EacRg11Unorm),
        native::WGPUTextureFormat_EACRG11Snorm => Some(wgt::TextureFormat::EacRg11Snorm),
        native::WGPUTextureFormat_ASTC4x4Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC4x4UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC5x4Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC5x4UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC5x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC5x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC6x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC6x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC6x6Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC6x6UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC8x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC8x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC8x6Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC8x6UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC8x8Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC8x8UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x5Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x5UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x6Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x6UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x8Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x8UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC10x10Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC10x10UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC12x10Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC12x10UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::UnormSrgb }),
        native::WGPUTextureFormat_ASTC12x12Unorm => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::Unorm }),
        native::WGPUTextureFormat_ASTC12x12UnormSrgb => Some(wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::UnormSrgb }),

        // wgpu.h extended
        native::WGPUNativeTextureFormat_R16Unorm => Some(wgt::TextureFormat::R16Unorm),
        native::WGPUNativeTextureFormat_R16Snorm => Some(wgt::TextureFormat::R16Snorm),
        native::WGPUNativeTextureFormat_Rg16Unorm => Some(wgt::TextureFormat::Rg16Unorm),
        native::WGPUNativeTextureFormat_Rg16Snorm => Some(wgt::TextureFormat::Rg16Snorm),
        native::WGPUNativeTextureFormat_Rgba16Unorm => Some(wgt::TextureFormat::Rgba16Unorm),
        native::WGPUNativeTextureFormat_Rgba16Snorm => Some(wgt::TextureFormat::Rgba16Snorm),
        native::WGPUNativeTextureFormat_NV12  => Some(wgt::TextureFormat::NV12),
        _ => panic!("Unknown texture format"),
    }
}

#[rustfmt::skip]
#[inline]
pub fn to_native_texture_format(rs_type: wgt::TextureFormat) -> Option<native::WGPUTextureFormat> {
    use wgt::{AstcBlock, AstcChannel};

    match rs_type {
        // unimplemented in webgpu.h
        wgt::TextureFormat::Astc { block:_, channel: AstcChannel::Hdr } => None,
        wgt::TextureFormat::R64Uint => None,

        wgt::TextureFormat::R8Unorm => Some(native::WGPUTextureFormat_R8Unorm),
        wgt::TextureFormat::R8Snorm => Some(native::WGPUTextureFormat_R8Snorm),
        wgt::TextureFormat::R8Uint => Some(native::WGPUTextureFormat_R8Uint),
        wgt::TextureFormat::R8Sint => Some(native::WGPUTextureFormat_R8Sint),
        wgt::TextureFormat::R16Uint => Some(native::WGPUTextureFormat_R16Uint),
        wgt::TextureFormat::R16Sint => Some(native::WGPUTextureFormat_R16Sint),
        wgt::TextureFormat::R16Float => Some(native::WGPUTextureFormat_R16Float),
        wgt::TextureFormat::Rg8Unorm => Some(native::WGPUTextureFormat_RG8Unorm),
        wgt::TextureFormat::Rg8Snorm => Some(native::WGPUTextureFormat_RG8Snorm),
        wgt::TextureFormat::Rg8Uint => Some(native::WGPUTextureFormat_RG8Uint),
        wgt::TextureFormat::Rg8Sint => Some(native::WGPUTextureFormat_RG8Sint),
        wgt::TextureFormat::R32Float => Some(native::WGPUTextureFormat_R32Float),
        wgt::TextureFormat::R32Uint => Some(native::WGPUTextureFormat_R32Uint),
        wgt::TextureFormat::R32Sint => Some(native::WGPUTextureFormat_R32Sint),
        wgt::TextureFormat::Rg16Uint => Some(native::WGPUTextureFormat_RG16Uint),
        wgt::TextureFormat::Rg16Sint => Some(native::WGPUTextureFormat_RG16Sint),
        wgt::TextureFormat::Rg16Float => Some(native::WGPUTextureFormat_RG16Float),
        wgt::TextureFormat::Rgba8Unorm => Some(native::WGPUTextureFormat_RGBA8Unorm),
        wgt::TextureFormat::Rgba8UnormSrgb => Some(native::WGPUTextureFormat_RGBA8UnormSrgb),
        wgt::TextureFormat::Rgba8Snorm => Some(native::WGPUTextureFormat_RGBA8Snorm),
        wgt::TextureFormat::Rgba8Uint => Some(native::WGPUTextureFormat_RGBA8Uint),
        wgt::TextureFormat::Rgba8Sint => Some(native::WGPUTextureFormat_RGBA8Sint),
        wgt::TextureFormat::Bgra8Unorm => Some(native::WGPUTextureFormat_BGRA8Unorm),
        wgt::TextureFormat::Bgra8UnormSrgb => Some(native::WGPUTextureFormat_BGRA8UnormSrgb),
        wgt::TextureFormat::Rgb10a2Uint => Some(native::WGPUTextureFormat_RGB10A2Uint),
        wgt::TextureFormat::Rgb10a2Unorm => Some(native::WGPUTextureFormat_RGB10A2Unorm),
        wgt::TextureFormat::Rg11b10Ufloat => Some(native::WGPUTextureFormat_RG11B10Ufloat),
        wgt::TextureFormat::Rgb9e5Ufloat => Some(native::WGPUTextureFormat_RGB9E5Ufloat),
        wgt::TextureFormat::Rg32Float => Some(native::WGPUTextureFormat_RG32Float),
        wgt::TextureFormat::Rg32Uint => Some(native::WGPUTextureFormat_RG32Uint),
        wgt::TextureFormat::Rg32Sint => Some(native::WGPUTextureFormat_RG32Sint),
        wgt::TextureFormat::Rgba16Uint => Some(native::WGPUTextureFormat_RGBA16Uint),
        wgt::TextureFormat::Rgba16Sint => Some(native::WGPUTextureFormat_RGBA16Sint),
        wgt::TextureFormat::Rgba16Float => Some(native::WGPUTextureFormat_RGBA16Float),
        wgt::TextureFormat::Rgba32Float => Some(native::WGPUTextureFormat_RGBA32Float),
        wgt::TextureFormat::Rgba32Uint => Some(native::WGPUTextureFormat_RGBA32Uint),
        wgt::TextureFormat::Rgba32Sint => Some(native::WGPUTextureFormat_RGBA32Sint),
        wgt::TextureFormat::Stencil8 => Some(native::WGPUTextureFormat_Stencil8),
        wgt::TextureFormat::Depth16Unorm => Some(native::WGPUTextureFormat_Depth16Unorm),
        wgt::TextureFormat::Depth24Plus => Some(native::WGPUTextureFormat_Depth24Plus),
        wgt::TextureFormat::Depth24PlusStencil8 => Some(native::WGPUTextureFormat_Depth24PlusStencil8),
        wgt::TextureFormat::Depth32Float => Some(native::WGPUTextureFormat_Depth32Float),
        wgt::TextureFormat::Depth32FloatStencil8 => Some(native::WGPUTextureFormat_Depth32FloatStencil8),
        wgt::TextureFormat::Bc1RgbaUnorm => Some(native::WGPUTextureFormat_BC1RGBAUnorm),
        wgt::TextureFormat::Bc1RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC1RGBAUnormSrgb),
        wgt::TextureFormat::Bc2RgbaUnorm => Some(native::WGPUTextureFormat_BC2RGBAUnorm),
        wgt::TextureFormat::Bc2RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC2RGBAUnormSrgb),
        wgt::TextureFormat::Bc3RgbaUnorm => Some(native::WGPUTextureFormat_BC3RGBAUnorm),
        wgt::TextureFormat::Bc3RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC3RGBAUnormSrgb),
        wgt::TextureFormat::Bc4RUnorm => Some(native::WGPUTextureFormat_BC4RUnorm),
        wgt::TextureFormat::Bc4RSnorm => Some(native::WGPUTextureFormat_BC4RSnorm),
        wgt::TextureFormat::Bc5RgUnorm => Some(native::WGPUTextureFormat_BC5RGUnorm),
        wgt::TextureFormat::Bc5RgSnorm => Some(native::WGPUTextureFormat_BC5RGSnorm),
        wgt::TextureFormat::Bc6hRgbUfloat => Some(native::WGPUTextureFormat_BC6HRGBUfloat),
        wgt::TextureFormat::Bc6hRgbFloat => Some(native::WGPUTextureFormat_BC6HRGBFloat),
        wgt::TextureFormat::Bc7RgbaUnorm => Some(native::WGPUTextureFormat_BC7RGBAUnorm),
        wgt::TextureFormat::Bc7RgbaUnormSrgb => Some(native::WGPUTextureFormat_BC7RGBAUnormSrgb),
        wgt::TextureFormat::Etc2Rgb8Unorm => Some(native::WGPUTextureFormat_ETC2RGB8Unorm),
        wgt::TextureFormat::Etc2Rgb8UnormSrgb => Some(native::WGPUTextureFormat_ETC2RGB8UnormSrgb),
        wgt::TextureFormat::Etc2Rgb8A1Unorm => Some(native::WGPUTextureFormat_ETC2RGB8A1Unorm),
        wgt::TextureFormat::Etc2Rgb8A1UnormSrgb => Some(native::WGPUTextureFormat_ETC2RGB8A1UnormSrgb),
        wgt::TextureFormat::Etc2Rgba8Unorm => Some(native::WGPUTextureFormat_ETC2RGBA8Unorm),
        wgt::TextureFormat::Etc2Rgba8UnormSrgb => Some(native::WGPUTextureFormat_ETC2RGBA8UnormSrgb),
        wgt::TextureFormat::EacR11Unorm => Some(native::WGPUTextureFormat_EACR11Unorm),
        wgt::TextureFormat::EacR11Snorm => Some(native::WGPUTextureFormat_EACR11Snorm),
        wgt::TextureFormat::EacRg11Unorm => Some(native::WGPUTextureFormat_EACRG11Unorm),
        wgt::TextureFormat::EacRg11Snorm => Some(native::WGPUTextureFormat_EACRG11Snorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC4x4Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B4x4, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC4x4UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC5x4Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x4, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC5x4UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC5x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B5x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC5x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC6x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC6x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC6x6Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B6x6, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC6x6UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC8x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC8x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC8x6Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x6, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC8x6UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC8x8Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B8x8, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC8x8UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x5Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x5, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x5UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x6Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x6, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x6UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x8Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x8, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x8UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC10x10Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B10x10, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC10x10UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC12x10Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x10, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC12x10UnormSrgb),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::Unorm } => Some(native::WGPUTextureFormat_ASTC12x12Unorm),
        wgt::TextureFormat::Astc { block: AstcBlock::B12x12, channel: AstcChannel::UnormSrgb } => Some(native::WGPUTextureFormat_ASTC12x12UnormSrgb),

        // wgpu.h extended
        wgt::TextureFormat::R16Unorm => Some(native::WGPUNativeTextureFormat_R16Unorm),
        wgt::TextureFormat::R16Snorm => Some(native::WGPUNativeTextureFormat_R16Snorm),
        wgt::TextureFormat::Rg16Unorm => Some(native::WGPUNativeTextureFormat_Rg16Unorm),
        wgt::TextureFormat::Rg16Snorm => Some(native::WGPUNativeTextureFormat_Rg16Snorm),
        wgt::TextureFormat::Rgba16Unorm => Some(native::WGPUNativeTextureFormat_Rgba16Unorm),
        wgt::TextureFormat::Rgba16Snorm => Some(native::WGPUNativeTextureFormat_Rgba16Snorm),
        wgt::TextureFormat::NV12 => Some(native::WGPUNativeTextureFormat_NV12),
        wgt::TextureFormat::P010 => Some(native::WGPUNativeTextureFormat_P010)
    }
}
