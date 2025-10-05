use super::map_origin3d;
use crate::{map_enum_with_undefined, native};

map_enum_with_undefined!(
    map_texture_aspect,
    WGPUTextureAspect,
    wgt::TextureAspect,
    "Unknown texture aspect",
    All,
    StencilOnly,
    DepthOnly
);

#[inline]
pub unsafe fn map_image_copy_texture(
    native: &native::WGPUTexelCopyTextureInfo,
) -> wgc::command::TexelCopyTextureInfo {
    wgt::TexelCopyTextureInfo {
        texture: native
            .texture
            .as_ref()
            .expect("invalid texture for image copy texture")
            .id,
        mip_level: native.mipLevel,
        origin: map_origin3d(&native.origin),
        aspect: map_texture_aspect(native.aspect).unwrap_or(wgt::TextureAspect::All),
    }
}

#[inline]
pub unsafe fn map_image_copy_buffer(
    native: &native::WGPUTexelCopyBufferInfo,
) -> wgc::command::TexelCopyBufferInfo {
    wgt::TexelCopyBufferInfo {
        buffer: native
            .buffer
            .as_ref()
            .expect("invalid buffer for image copy buffer")
            .id,
        layout: map_texture_data_layout(&native.layout),
    }
}

#[inline]
pub fn map_texture_data_layout(
    native: &native::WGPUTexelCopyBufferLayout,
) -> wgt::TexelCopyBufferLayout {
    wgt::TexelCopyBufferLayout {
        offset: native.offset,
        bytes_per_row: match native.bytesPerRow {
            0 => panic!("invalid bytesPerRow"),
            native::WGPU_COPY_STRIDE_UNDEFINED => None,
            _ => Some(native.bytesPerRow),
        },
        rows_per_image: match native.rowsPerImage {
            0 => panic!("invalid rowsPerImage"),
            native::WGPU_COPY_STRIDE_UNDEFINED => None,
            _ => Some(native.rowsPerImage),
        },
    }
}
