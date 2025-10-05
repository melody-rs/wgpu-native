use super::{from_u64_bits, map_texture_format};
use crate::utils::make_slice;
use crate::{map_enum_with_undefined, native};
use std::num::{NonZeroU32, NonZeroU64};

// These are defined as UINT64_MAX in the header, but bindgen currently can't process that define.
// See https://github.com/rust-lang/rust-bindgen/issues/2822
pub const WGPU_WHOLE_SIZE: u64 = u64::MAX;

map_enum_with_undefined!(
    map_storage_texture_access,
    WGPUStorageTextureAccess,
    wgt::StorageTextureAccess,
    "Unknown storage texture access",
    WriteOnly,
    ReadOnly,
    ReadWrite
);

#[inline]
pub fn map_bind_group_entry<'a>(
    entry: &'a native::WGPUBindGroupEntry,
    extras: Option<&native::WGPUBindGroupEntryExtras>,
) -> wgc::binding_model::BindGroupEntry<'a> {
    if let Some(buffer) = unsafe { entry.buffer.as_ref() } {
        return wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: wgc::binding_model::BindingResource::Buffer(
                wgc::binding_model::BufferBinding {
                    buffer: buffer.id,
                    offset: entry.offset,
                    size: match entry.size {
                        0 => panic!("invalid size"),
                        WGPU_WHOLE_SIZE => None,
                        _ => Some(unsafe { NonZeroU64::new_unchecked(entry.size) }),
                    },
                },
            ),
        };
    } else if let Some(sampler) = unsafe { entry.sampler.as_ref() } {
        return wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: wgc::binding_model::BindingResource::Sampler(sampler.id),
        };
    } else if let Some(texture_view) = unsafe { entry.textureView.as_ref() } {
        return wgc::binding_model::BindGroupEntry {
            binding: entry.binding,
            resource: wgc::binding_model::BindingResource::TextureView(texture_view.id),
        };
    } else if let Some(extras) = extras {
        if let Some(texture_views) = unsafe { extras.textureViews.as_ref() } {
            let arr = make_slice(texture_views, extras.textureViewCount)
                .iter()
                .map(|v| {
                    unsafe { v.as_ref() }
                        .expect("invalid texture views for bind group entry extras")
                        .id
                })
                .collect();
            return wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::TextureViewArray(arr),
            };
        } else if let Some(samplers) = unsafe { extras.samplers.as_ref() } {
            let arr = make_slice(samplers, extras.samplerCount)
                .iter()
                .map(|v| {
                    unsafe { v.as_ref() }
                        .expect("invalid sampler for bind group entry extras")
                        .id
                })
                .collect();
            return wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::SamplerArray(arr),
            };
        } else if let Some(buffers) = unsafe { extras.buffers.as_ref() } {
            let arr = make_slice(buffers, extras.bufferCount)
                .iter()
                .map(|v| wgc::binding_model::BufferBinding {
                    buffer: unsafe { v.as_ref() }
                        .expect("invalid buffers for bind group entry extras")
                        .id,
                    offset: entry.offset,
                    size: std::num::NonZeroU64::new(entry.size),
                })
                .collect();
            return wgc::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: wgc::binding_model::BindingResource::BufferArray(arr),
            };
        }
    }

    panic!("invalid bind group entry for bind group descriptor");
}

#[inline]
pub fn map_bind_group_layout_entry(
    entry: &native::WGPUBindGroupLayoutEntry,
    extras: Option<&native::WGPUBindGroupLayoutEntryExtras>,
) -> wgt::BindGroupLayoutEntry {
    let is_buffer = entry.buffer.type_ != native::WGPUBufferBindingType_BindingNotUsed;
    let is_sampler = entry.sampler.type_ != native::WGPUSamplerBindingType_BindingNotUsed;
    let is_texture = entry.texture.sampleType != native::WGPUTextureSampleType_BindingNotUsed;
    let is_storage_texture =
        entry.storageTexture.access != native::WGPUStorageTextureAccess_BindingNotUsed;

    // TODO extract most of these into custom map functions?
    let ty = if is_texture {
        wgt::BindingType::Texture {
            sample_type: match entry.texture.sampleType {
                native::WGPUTextureSampleType_Float | native::WGPUTextureSampleType_Undefined => {
                    wgt::TextureSampleType::Float { filterable: true }
                }
                native::WGPUTextureSampleType_UnfilterableFloat => {
                    wgt::TextureSampleType::Float { filterable: false }
                }
                native::WGPUTextureSampleType_Depth => wgt::TextureSampleType::Depth,
                native::WGPUTextureSampleType_Sint => wgt::TextureSampleType::Sint,
                native::WGPUTextureSampleType_Uint => wgt::TextureSampleType::Uint,
                _ => panic!(
                    "invalid sample type for texture binding layout at binding {}",
                    entry.binding
                ),
            },
            view_dimension: match entry.texture.viewDimension {
                native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                native::WGPUTextureViewDimension_2DArray => wgt::TextureViewDimension::D2Array,
                native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                native::WGPUTextureViewDimension_CubeArray => wgt::TextureViewDimension::CubeArray,
                native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                _ => panic!(
                    "invalid texture view dimension for texture binding layout at binding {}",
                    entry.binding
                ),
            },
            multisampled: entry.texture.multisampled != 0,
        }
    } else if is_sampler {
        match entry.sampler.type_ {
            native::WGPUSamplerBindingType_Filtering | native::WGPUSamplerBindingType_Undefined => {
                wgt::BindingType::Sampler(wgt::SamplerBindingType::Filtering)
            }
            native::WGPUSamplerBindingType_NonFiltering => {
                wgt::BindingType::Sampler(wgt::SamplerBindingType::NonFiltering)
            }
            native::WGPUSamplerBindingType_Comparison => {
                wgt::BindingType::Sampler(wgt::SamplerBindingType::Comparison)
            }
            _ => panic!(
                "invalid sampler binding type for sampler binding layout at binding {}",
                entry.binding
            ),
        }
    } else if is_storage_texture {
        wgt::BindingType::StorageTexture {
            access: map_storage_texture_access(entry.storageTexture.access)
                .unwrap_or(wgt::StorageTextureAccess::WriteOnly),
            format: map_texture_format(entry.storageTexture.format)
                .expect("invalid texture format for storage texture binding layout"),
            view_dimension: match entry.storageTexture.viewDimension {
                native::WGPUTextureViewDimension_1D => wgt::TextureViewDimension::D1,
                native::WGPUTextureViewDimension_2D => wgt::TextureViewDimension::D2,
                native::WGPUTextureViewDimension_2DArray => wgt::TextureViewDimension::D2Array,
                native::WGPUTextureViewDimension_Cube => wgt::TextureViewDimension::Cube,
                native::WGPUTextureViewDimension_CubeArray => wgt::TextureViewDimension::CubeArray,
                native::WGPUTextureViewDimension_3D => wgt::TextureViewDimension::D3,
                _ => {
                    panic!("invalid texture view dimension for storage texture binding layout at binding {}", entry.binding)
                }
            },
        }
    } else if is_buffer {
        wgt::BindingType::Buffer {
            ty: match entry.buffer.type_ {
                native::WGPUBufferBindingType_Uniform | native::WGPUBufferBindingType_Undefined => {
                    wgt::BufferBindingType::Uniform
                }
                native::WGPUBufferBindingType_Storage => {
                    wgt::BufferBindingType::Storage { read_only: false }
                }
                native::WGPUBufferBindingType_ReadOnlyStorage => {
                    wgt::BufferBindingType::Storage { read_only: true }
                }
                _ => panic!(
                    "invalid buffer binding type for buffer binding layout at binding {}",
                    entry.binding
                ),
            },
            has_dynamic_offset: entry.buffer.hasDynamicOffset != 0,
            min_binding_size: {
                assert_ne!(
                    entry.buffer.minBindingSize, WGPU_WHOLE_SIZE,
                    "invalid min binding size for buffer binding layout, use 0 instead"
                );

                NonZeroU64::new(entry.buffer.minBindingSize)
            },
        }
    } else {
        panic!("invalid bind group layout entry for bind group layout descriptor");
    };

    wgt::BindGroupLayoutEntry {
        ty,
        binding: entry.binding,
        visibility: from_u64_bits(entry.visibility)
            .expect("invalid visibility for bind group layout entry"),
        count: extras.and_then(|v| NonZeroU32::new(v.count)),
    }
}
