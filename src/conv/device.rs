use super::{map_features, map_required_limits};
use crate::utils::{make_slice, string_view_into_label};
use crate::{follow_chain, new_userdata};
use crate::{native, UncapturedErrorCallback};

#[inline]
pub(crate) unsafe fn map_device_descriptor<'a>(
    des: &native::WGPUDeviceDescriptor,
    base_limits: wgt::Limits,
    _extras: Option<&native::WGPUDeviceExtras>,
) -> (
    wgt::DeviceDescriptor<wgc::Label<'a>>,
    Option<UncapturedErrorCallback>,
) {
    (
        wgt::DeviceDescriptor {
            label: string_view_into_label(des.label),
            required_features: map_features(make_slice(
                des.requiredFeatures,
                des.requiredFeatureCount,
            )),
            required_limits: match unsafe { des.requiredLimits.as_ref() } {
                Some(required_limits) => unsafe {
                    follow_chain!(
                        map_required_limits((required_limits, base_limits),
                        WGPUSType_NativeLimits => native::WGPUNativeLimits)
                    )
                },
                None => base_limits,
            },
            // TODO(wgpu.h)
            memory_hints: Default::default(),
            trace: Default::default(),
            experimental_features: wgt::ExperimentalFeatures::disabled(),
        },
        match des.uncapturedErrorCallbackInfo.callback {
            None => None,
            callback => Some(UncapturedErrorCallback {
                callback,
                userdata: new_userdata!(des.uncapturedErrorCallbackInfo),
            }),
        },
    )
}
