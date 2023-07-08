use crate::{route::RouteExt, state::State};
use eframe::{egui, epaint::vec2};

#[derive(Default)]
pub struct WifiQR;

impl RouteExt for WifiQR {
    fn ui(&mut self, state: &mut State, ui: &mut egui::Ui) {
        let texture = state.selected_wifi_qr_texture.get_or_insert_with(|| {
            let wifi = &state.selected_wifi;
            let mut qr = format!("WIFI:T:{};S:{};", wifi.conn_type, wifi.name);
            if !wifi.password.is_empty() {
                qr.push_str(&format!("P:{};", wifi.password))
            }
            qr.push_str(&format!("{};", if wifi.hidden { "H:true" } else { "" }));

            let png: Vec<u8> =
                qrcode_generator::to_png_to_vec(qr, qrcode_generator::QrCodeEcc::Low, 1024)
                    .unwrap();

            let img = image::load_from_memory_with_format(&png, image::ImageFormat::Png).unwrap();
            let size = [img.width() as _, img.height() as _];
            let img_buffer = img.to_rgba8();
            let pixels = img_buffer.as_flat_samples();

            ui.ctx().load_texture(
                "wifi-qr",
                egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                Default::default(),
            )
        });

        ui.centered_and_justified(|ui| {
            ui.image(texture, ui.available_size().min(vec2(300.0, 300.0)));
        });
    }
}
