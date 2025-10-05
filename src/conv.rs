use crate::native;

// TODO move conversions handled into lib.rs into conv
mod instance;
pub use instance::*;

mod device;
pub(crate) use device::*;

mod features;
pub(crate) use features::*;

mod limits;
pub(crate) use limits::*;

mod bind_group;
pub(crate) use bind_group::*;

mod texture;
pub(crate) use texture::*;

mod surface;
pub(crate) use surface::*;

mod queue;
pub(crate) use queue::*;

mod sampler;
pub(crate) use sampler::*;

mod pipeline;
pub(crate) use pipeline::*;

mod render_pass;
pub(crate) use render_pass::*;

mod query_set;
pub(crate) use query_set::*;

mod adapter;
pub(crate) use adapter::*;

// it's SIZE_MAX in headers but it's not available in some compilers
pub const WGPU_WHOLE_MAP_SIZE: usize = usize::MAX;
pub const WGPU_STRLEN: usize = usize::MAX;

// TODO find a spot for these?
#[inline]
pub fn map_extent3d(native: &native::WGPUExtent3D) -> wgt::Extent3d {
    wgt::Extent3d {
        width: native.width,
        height: native.height,
        depth_or_array_layers: native.depthOrArrayLayers,
    }
}

#[inline]
pub fn map_origin3d(native: &native::WGPUOrigin3D) -> wgt::Origin3d {
    wgt::Origin3d {
        x: native.x,
        y: native.y,
        z: native.z,
    }
}

#[inline]
pub fn map_storage_report(report: &wgc::registry::RegistryReport) -> native::WGPURegistryReport {
    native::WGPURegistryReport {
        numAllocated: report.num_allocated,
        numKeptFromUser: report.num_kept_from_user,
        numReleasedFromUser: report.num_released_from_user,
        elementSize: report.element_size,
    }
}

#[inline]
pub fn map_hub_report(report: &wgc::hub::HubReport) -> native::WGPUHubReport {
    native::WGPUHubReport {
        adapters: map_storage_report(&report.adapters),
        devices: map_storage_report(&report.devices),
        queues: map_storage_report(&report.queues),
        pipelineLayouts: map_storage_report(&report.pipeline_layouts),
        shaderModules: map_storage_report(&report.shader_modules),
        bindGroupLayouts: map_storage_report(&report.bind_group_layouts),
        bindGroups: map_storage_report(&report.bind_groups),
        commandBuffers: map_storage_report(&report.command_buffers),
        renderBundles: map_storage_report(&report.render_bundles),
        renderPipelines: map_storage_report(&report.render_pipelines),
        computePipelines: map_storage_report(&report.compute_pipelines),
        pipelineCaches: map_storage_report(&report.pipeline_caches),
        querySets: map_storage_report(&report.query_sets),
        buffers: map_storage_report(&report.buffers),
        textures: map_storage_report(&report.textures),
        textureViews: map_storage_report(&report.texture_views),
        samplers: map_storage_report(&report.samplers),
    }
}

#[inline]
pub fn write_global_report(
    native_report: &mut native::WGPUGlobalReport,
    report: &wgc::global::GlobalReport,
) {
    native_report.surfaces = map_storage_report(&report.surfaces);
    native_report.hub = map_hub_report(&report.hub);
}

pub fn from_u64_bits<T: bitflags::Flags<Bits = u32>>(value: u64) -> Option<T> {
    if value > u32::MAX.into() {
        return None;
    }

    T::from_bits(value as u32)
}
