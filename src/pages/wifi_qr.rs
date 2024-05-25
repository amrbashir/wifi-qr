use crate::{route::RouteExt, state::State};
use eframe::egui::{self, ImageSource, Layout};

#[derive(Default)]
pub struct WifiQR;

impl RouteExt for WifiQR {
    fn ui(&mut self, state: &mut State, ui: &mut egui::Ui) {
        let bytes = state
            .selected_wifi_qr_img_bytes
            .get_or_insert_with(|| state.selected_wifi.qr())
            .clone();

        let height = ui.available_height();

        ui.add_space(height / 4.0);
        ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(height / 4.0);
            ui.image(ImageSource::Bytes {
                uri: format!("bytes://wifi_{}_qr", state.selected_wifi.name).into(),
                bytes: bytes.into(),
            });
        });
    }
}
