use crate::native;
use crate::utils::{make_slice, string_view_into_label};

pub fn map_query_set_index(index: u32) -> Option<u32> {
    match index {
        native::WGPU_QUERY_SET_INDEX_UNDEFINED => None,
        _ => Some(index),
    }
}

#[inline]
pub unsafe fn map_query_set_descriptor<'a>(
    desc: &native::WGPUQuerySetDescriptor,
    extras: Option<&native::WGPUQuerySetDescriptorExtras>,
) -> wgt::QuerySetDescriptor<wgc::Label<'a>> {
    wgt::QuerySetDescriptor {
        label: string_view_into_label(desc.label),
        count: desc.count,
        ty: match (desc.type_, extras) {
            (native::WGPUQueryType_Occlusion, _) => wgt::QueryType::Occlusion,
            (native::WGPUQueryType_Timestamp, _) => wgt::QueryType::Timestamp,
            (native::WGPUNativeQueryType_PipelineStatistics, Some(extras)) => {
                let mut types = wgt::PipelineStatisticsTypes::empty();

                make_slice(extras.pipelineStatistics, extras.pipelineStatisticCount)
                    .iter()
                    .for_each(|f| {
                        types.insert(match *f {
                            native::WGPUPipelineStatisticName_VertexShaderInvocations => {
                                wgt::PipelineStatisticsTypes::VERTEX_SHADER_INVOCATIONS
                            }
                            native::WGPUPipelineStatisticName_ClipperInvocations => {
                                wgt::PipelineStatisticsTypes::CLIPPER_INVOCATIONS
                            }
                            native::WGPUPipelineStatisticName_ClipperPrimitivesOut => {
                                wgt::PipelineStatisticsTypes::CLIPPER_PRIMITIVES_OUT
                            }
                            native::WGPUPipelineStatisticName_FragmentShaderInvocations => {
                                wgt::PipelineStatisticsTypes::FRAGMENT_SHADER_INVOCATIONS
                            }
                            native::WGPUPipelineStatisticName_ComputeShaderInvocations => {
                                wgt::PipelineStatisticsTypes::COMPUTE_SHADER_INVOCATIONS
                            }
                            _ => panic!("invalid pipeline statistics name"),
                        });
                    });

                wgt::QueryType::PipelineStatistics(types)
            }
            _ => panic!("invalid query type"),
        },
    }
}
