use freya::prelude::*;

use crate::wifi::WiFi;

#[allow(non_snake_case)]
#[component]
pub fn Qr(wifi: WiFi) -> Element {
    let qr = wifi.qr();
    let image_data = dynamic_bytes(qr);

    let theme = use_theme();
    let is_dark = theme.read().name == crate::theme::DARK_THEME.name;

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            cross_align: "center",
            main_align: "center",
            image {
                width: "300",
                height: "300",
                image_data,
            },
            label {
                font_size: "12",
                color: if is_dark { "rgb(255, 255, 255, 0.75)" } else { "rgb(95, 95, 95)" },
                "Password: " {wifi.password}
            }
        }
    )
}
