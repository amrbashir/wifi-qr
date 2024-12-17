#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{num::NonZeroIsize, sync::Arc};

use dioxus_router::prelude::*;
use freya::prelude::*;
use freya_engine::prelude::Color;
use winit::{platform::windows::IconExtWindows, raw_window_handle::*};

use crate::pages::Route;

mod components;
mod icons;
mod pages;
mod theme;
mod wifi;

#[derive(Clone, Copy)]
pub struct Hwnd(isize);

impl HasWindowHandle for Hwnd {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(unsafe {
            WindowHandle::borrow_raw(RawWindowHandle::Win32(Win32WindowHandle::new(
                NonZeroIsize::new_unchecked(self.0),
            )))
        })
    }
}

static mut MAIN_WINDOW_HWND: Hwnd = Hwnd(0);

fn app() -> Element {
    rsx!(Router::<Route> {})
}

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                icon: winit::window::Icon::from_resource(1, None).ok(),
                title: "WiFi QR",
                size: (400.0, 400.0),
                min_size: Some((400.0, 400.0)),
                transparent: true,
                background: Color::TRANSPARENT,
                #[cfg(windows)]
                on_setup: Some(Arc::new(Box::new(move |window| {
                    if let Ok(RawWindowHandle::Win32(handle)) =
                        window.window_handle().map(|h| h.as_raw())
                    {
                        let hwnd = Hwnd(handle.hwnd.get());
                        let _ = window_vibrancy::apply_mica(hwnd, Some(true));
                        unsafe { MAIN_WINDOW_HWND = hwnd }
                    }
                }))),
                ..Default::default()
            },
            ..Default::default()
        },
    );
}
