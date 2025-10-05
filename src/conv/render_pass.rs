use crate::map_enum_with_undefined;
use crate::native;

map_enum_with_undefined!(
    map_store_op,
    WGPUStoreOp,
    wgc::command::StoreOp,
    "Unknown store operation",
    Discard,
    Store
);

#[inline]
pub fn map_load_op<T>(
    command: native::WGPULoadOp,
    clear_value: T,
) -> Option<wgc::command::LoadOp<T>> {
    match command {
        native::WGPULoadOp_Load => Some(wgc::command::LoadOp::Load),
        native::WGPULoadOp_Clear => Some(wgc::command::LoadOp::Clear(clear_value)),
        _ => None,
    }
}

#[inline]
pub fn map_color(native: &native::WGPUColor) -> wgt::Color {
    wgt::Color {
        r: native.r,
        g: native.g,
        b: native.b,
        a: native.a,
    }
}
