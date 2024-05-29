use dioxus_router::prelude::*;
use freya::prelude::*;

use crate::{components::FluentButton, icons, wifi::WiFi, Route, MAIN_WINDOW_HWND};

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub margin: Option<String>,
}

#[allow(non_snake_case)]
pub fn NavBar(props: NavBarProps) -> Element {
    let router = router();
    let current_route = use_route::<Route>();
    let is_home = matches!(current_route, Route::Home {});

    let mut theme = use_theme();
    let is_dark = theme.read().name == crate::theme::DARK_THEME.name;
    let change_theme = move |_| {
        *theme.write() = if is_dark {
            let _ = window_vibrancy::apply_mica(unsafe { MAIN_WINDOW_HWND }, Some(false));
            crate::theme::LIGHT_THEME
        } else {
            let _ = window_vibrancy::apply_mica(unsafe { MAIN_WINDOW_HWND }, Some(true));
            crate::theme::DARK_THEME
        };
    };

    let mut wifi_list = use_context::<Signal<Vec<WiFi>>>();

    let margin = props.margin.as_deref().unwrap_or("0");

    rsx!(
        rect {
            margin,
            direction: "horizontal",
            width: "fill",
            FluentButton {
                flat: true,
                enabled: !is_home,
                onclick: move |_| { router.go_back() },
                label {{icons::BACK}}
            }
            rect {
                direction: "horizontal",
                width: "fill",
                main_align: "end",
                if is_home {
                    FluentButton {
                        margin: "0 4 0 0",
                        flat: true,
                        onclick: move |_| {
                            use_future( move || async move {
                                *wifi_list.write() = WiFi::all();
                            });
                        },
                        label {{icons::REFRESH}}
                    }
                }
                FluentButton {
                    flat: true,
                    onclick: change_theme,
                    label {
                        if is_dark {
                            {icons::SUN}
                        } else {
                            {icons::MOON}
                        }
                    }
                }
            }
        }
    )
}
