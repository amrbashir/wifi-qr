use crate::{route::Route, wifi::WiFi};

pub struct State {
    pub next_route: Option<Route>,
    pub selected_route: Route,
    pub previous_route: Option<Route>,
    pub wifi_list: Vec<WiFi>,
    pub selected_wifi: WiFi,
    pub requested_refresh: bool,
    pub selected_wifi_qr_img_bytes: Option<Vec<u8>>,
    pub dark_mode: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_route: Default::default(),
            selected_route: Default::default(),
            previous_route: Default::default(),
            wifi_list: Default::default(),
            selected_wifi: Default::default(),
            requested_refresh: true,
            selected_wifi_qr_img_bytes: None,
            dark_mode: true,
        }
    }
}
