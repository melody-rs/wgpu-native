use super::{map_texture_format, map_texture_usage_flags};
use crate::utils::make_slice;
use crate::{map_enum, map_enum_with_undefined, native};
use std::num::{NonZeroIsize, NonZeroU32};
use std::ptr::NonNull;

map_enum_with_undefined!(
    map_present_mode,
    WGPUPresentMode,
    wgt::PresentMode,
    "Unknown present mode",
    Immediate,
    Mailbox,
    Fifo,
    FifoRelaxed
);

map_enum!(
    map_composite_alpha_mode,
    WGPUCompositeAlphaMode,
    wgt::CompositeAlphaMode,
    Auto: Auto,
    Opaque: Opaque,
    Premultiplied: PreMultiplied,
    Unpremultiplied: PostMultiplied,
    Inherit: Inherit
);

pub enum CreateSurfaceParams {
    Raw(
        (
            raw_window_handle::RawDisplayHandle,
            raw_window_handle::RawWindowHandle,
        ),
    ),
    #[cfg(all(any(target_os = "ios", target_os = "macos"), feature = "metal"))]
    Metal(*mut std::ffi::c_void),
    #[cfg(all(target_os = "windows", feature = "dx12"))]
    SwapChainPanel(*mut std::ffi::c_void),
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn map_surface(
    _: &native::WGPUSurfaceDescriptor,
    win: Option<&native::WGPUSurfaceSourceWindowsHWND>,
    xcb: Option<&native::WGPUSurfaceSourceXCBWindow>,
    xlib: Option<&native::WGPUSurfaceSourceXlibWindow>,
    wl: Option<&native::WGPUSurfaceSourceWaylandSurface>,
    _metal: Option<&native::WGPUSurfaceSourceMetalLayer>,
    android: Option<&native::WGPUSurfaceSourceAndroidNativeWindow>,
    _swap_chain_panel: Option<&native::WGPUSurfaceSourceSwapChainPanel>,
) -> CreateSurfaceParams {
    if let Some(win) = win {
        let display_handle = raw_window_handle::WindowsDisplayHandle::new();
        let mut window_handle =
            raw_window_handle::Win32WindowHandle::new(NonZeroIsize::new_unchecked(win.hwnd as _));
        window_handle.hinstance = NonZeroIsize::new(win.hinstance as _);

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Windows(display_handle),
            raw_window_handle::RawWindowHandle::Win32(window_handle),
        ));
    }

    if let Some(xcb) = xcb {
        let connection = NonNull::<std::ffi::c_void>::new_unchecked(xcb.connection);
        let display_handle = raw_window_handle::XcbDisplayHandle::new(Some(connection), 0);
        let window_handle =
            raw_window_handle::XcbWindowHandle::new(NonZeroU32::new_unchecked(xcb.window));

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Xcb(display_handle),
            raw_window_handle::RawWindowHandle::Xcb(window_handle),
        ));
    }

    if let Some(xlib) = xlib {
        let display = NonNull::<std::ffi::c_void>::new_unchecked(xlib.display);
        let display_handle = raw_window_handle::XlibDisplayHandle::new(Some(display), 0);
        let window_handle = raw_window_handle::XlibWindowHandle::new(xlib.window as _);

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Xlib(display_handle),
            raw_window_handle::RawWindowHandle::Xlib(window_handle),
        ));
    }

    if let Some(wl) = wl {
        let display = NonNull::<std::ffi::c_void>::new_unchecked(wl.display);
        let surface = NonNull::<std::ffi::c_void>::new_unchecked(wl.surface);
        let display_handle = raw_window_handle::WaylandDisplayHandle::new(display);
        let window_handle = raw_window_handle::WaylandWindowHandle::new(surface);

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Wayland(display_handle),
            raw_window_handle::RawWindowHandle::Wayland(window_handle),
        ));
    }

    #[cfg(all(any(target_os = "ios", target_os = "macos"), feature = "metal"))]
    if let Some(metal) = _metal {
        return CreateSurfaceParams::Metal(metal.layer);
    }

    if let Some(android) = android {
        let display_handle = raw_window_handle::AndroidDisplayHandle::new();
        let window_handle =
            raw_window_handle::AndroidNdkWindowHandle::new(NonNull::new_unchecked(android.window));

        return CreateSurfaceParams::Raw((
            raw_window_handle::RawDisplayHandle::Android(display_handle),
            raw_window_handle::RawWindowHandle::AndroidNdk(window_handle),
        ));
    }

    #[cfg(all(target_os = "windows", feature = "dx12"))]
    if let Some(swap_chain_panel) = _swap_chain_panel {
        return CreateSurfaceParams::SwapChainPanel(swap_chain_panel.panelNative);
    }

    panic!("Error: Unsupported Surface");
}

#[inline]
pub fn map_surface_configuration(
    config: &native::WGPUSurfaceConfiguration,
    extras: Option<&native::WGPUSurfaceConfigurationExtras>,
) -> wgt::SurfaceConfiguration<Vec<wgt::TextureFormat>> {
    wgt::SurfaceConfiguration {
        usage: map_texture_usage_flags(config.usage as native::WGPUTextureUsage),
        format: map_texture_format(config.format)
            .expect("invalid format for surface configuration"),
        width: config.width,
        height: config.height,
        present_mode: map_present_mode(config.presentMode).unwrap_or(wgt::PresentMode::Fifo),
        alpha_mode: map_composite_alpha_mode(config.alphaMode)
            .expect("invalid alpha mode for surface configuration"),
        view_formats: make_slice(config.viewFormats, config.viewFormatCount)
            .iter()
            .map(|f| map_texture_format(*f).expect("invalid view format for surface configuration"))
            .collect(),
        desired_maximum_frame_latency: match extras {
            Some(extras) => extras.desiredMaximumFrameLatency,
            // Default is 2, https://github.com/gfx-rs/wgpu/blob/7b4cbc26192d6d56a31f8e67769e656a6627b222/wgpu/src/api/surface.rs#L87
            None => 2,
        },
    }
}

#[inline]
pub fn to_native_present_mode(mode: wgt::PresentMode) -> Option<native::WGPUPresentMode> {
    match mode {
        wgt::PresentMode::Fifo => Some(native::WGPUPresentMode_Fifo),
        wgt::PresentMode::Immediate => Some(native::WGPUPresentMode_Immediate),
        wgt::PresentMode::Mailbox => Some(native::WGPUPresentMode_Mailbox),
        wgt::PresentMode::FifoRelaxed => Some(native::WGPUPresentMode_FifoRelaxed),
        wgt::PresentMode::AutoVsync | wgt::PresentMode::AutoNoVsync => None,
    }
}

#[inline]
pub fn to_native_composite_alpha_mode(
    mode: wgt::CompositeAlphaMode,
) -> native::WGPUCompositeAlphaMode {
    match mode {
        wgt::CompositeAlphaMode::Auto => native::WGPUCompositeAlphaMode_Auto,
        wgt::CompositeAlphaMode::Opaque => native::WGPUCompositeAlphaMode_Opaque,
        wgt::CompositeAlphaMode::PreMultiplied => native::WGPUCompositeAlphaMode_Premultiplied,
        wgt::CompositeAlphaMode::PostMultiplied => native::WGPUCompositeAlphaMode_Unpremultiplied,
        wgt::CompositeAlphaMode::Inherit => native::WGPUCompositeAlphaMode_Inherit,
    }
}
