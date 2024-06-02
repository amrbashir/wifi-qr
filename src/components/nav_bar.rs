use dioxus_router::prelude::*;
use freya::prelude::*;

use crate::{components::FluentButton, icons, theme::ThemeVariant, wifi::WiFi, Route};

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub margin: Option<String>,
}

#[allow(non_snake_case)]
pub fn NavBar(props: NavBarProps) -> Element {
    let router = router();
    let current_route = use_route::<Route>();
    let is_home = matches!(current_route, Route::Home {});

    let mut current_theme = use_theme();
    let mut current_theme_variant: Signal<ThemeVariant> = use_signal(|| ThemeVariant::System);

    let next_theme_variant_icon = current_theme_variant.read().next_icon();

    let change_theme = move |_| {
        let next_variant = current_theme_variant.read().next();
        *current_theme_variant.write() = next_variant;
        *current_theme.write() = next_variant.into();
        next_variant.apply_mica();
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
                    label {{next_theme_variant_icon}}
                }
            }
        }
    )
}
