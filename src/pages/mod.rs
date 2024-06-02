use dioxus_router::prelude::*;
use freya::prelude::*;

mod home;
mod qr;

pub use home::*;
pub use qr::*;

use crate::{components::NavBar, wifi::WiFi};

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/:wifi/qr")]
        Qr { wifi: WiFi },
    #[end_layout]
    #[route("/..route")]
    PageNotFound {},
}

#[allow(non_snake_case)]
fn PageNotFound() -> Element {
    rsx!(
        rect {
            main_align: "center",
            width: "fill",
            height: "fill",
            label {
                "Page Not Found, How did this happen?"
            }
        }
    )
}

#[allow(non_snake_case)]
fn Layout() -> Element {
    crate::theme::use_init_theme();

    use_context_provider(|| Signal::new(Vec::<WiFi>::new()));

    let router = router();

    let onkeydown = move |e: KeyboardEvent| {
        if e.modifiers.alt() {
            if e.key == Key::ArrowLeft {
                router.go_back();
            } else if e.key == Key::ArrowRight {
                router.go_forward();
            }
        }
    };

    rsx!(
        rect {
            onkeydown,
            padding: "0 8 8 8",
            font_family: "Segoe UI Variable, Segoe Fluent Icons",
            NavBar {
                margin: "0 0 8 0",
            },
            NativeRouter {
                Outlet::<Route> {  }
            }
        }
    )
}
