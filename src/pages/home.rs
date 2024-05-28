use dioxus_router::prelude::*;
use freya::prelude::*;

use crate::{components::WinUiButton, icons, wifi::WiFi, Route};

#[allow(non_snake_case)]
#[component]
pub fn Home() -> Element {
    let router = router();

    let mut wifi_list = use_context::<Signal<Vec<WiFi>>>();
    use_effect(move || {
        *wifi_list.write() = WiFi::all();
    });

    let theme = use_theme();
    let is_dark = theme.read().name == crate::theme::DARK_THEME.name;

    rsx!(
        ScrollView {
            for wifi in wifi_list() {
                WinUiButton {
                    direction: "horizontal",
                    main_align: "start",
                    width: "fill",
                    height: "60",
                    margin: "0 0 4 0",
                    padding: "8 16",
                    label {
                        font_size: "24",
                        margin: "0 8 0 0",
                        {icons::WIFI}
                    },
                    rect {
                        label {{ wifi.name}},
                        label {
                            font_size: "10",
                            color: if is_dark { "rgb(255, 255, 255, 0.75)" } else { "rgb(95, 95, 95)" },
                            {format!("Connection: {}{}", wifi.conn_type, if wifi.hidden { " | Hidden"} else { "" })}
                        },
                    }
                    onclick: {
                        to_owned![wifi];
                        move |_| {
                            router.push(Route::Qr { wifi: wifi.clone() });
                        }
                    },
                }
            }
        }
    )
}
