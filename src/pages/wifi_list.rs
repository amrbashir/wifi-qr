use std::{os::windows::process::CommandExt, process::Command};

use eframe::egui;

use crate::{
    route::{Route, RouteExt},
    state::State,
    widgets,
    wifi::{ConnectionType, WiFi},
};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Default)]
pub struct WifiList;

impl RouteExt for WifiList {
    fn ui(&mut self, state: &mut State, ui: &mut egui::Ui) {
        if state.requested_refresh {
            update_wifi_list(&mut state.wifi_list);
            state.requested_refresh = false;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.max_rect().width());
            ui.with_layout(
                egui::Layout::top_down_justified(eframe::emath::Align::Min),
                |ui| {
                    for wifi in state.wifi_list.clone() {
                        if ui
                            .add(
                                widgets::Button::icon_and_text("", wifi.name.as_str()).subtitle(
                                    format!(
                                        "Connection: {}{}",
                                        wifi.conn_type,
                                        if wifi.hidden { " | Hidden" } else { "" }
                                    ),
                                ),
                            )
                            .clicked()
                        {
                            state.selected_wifi = wifi;
                            state.selected_wifi_qr_img_bytes = None;
                            Route::navigate(state, Route::WifiQR);
                        }
                    }
                },
            );
        });
    }
}

fn update_wifi_list(wifi_list: &mut Vec<WiFi>) {
    let profiles: Vec<String> = Command::new("netsh")
        .args(["wlan", "show", "profiles"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()
        .map(|o| {
            let output = String::from_utf8_lossy(&o.stdout);
            output
                .lines()
                .filter(|l| l.contains("All User Profile"))
                .map(|l| l.split_once(':').unwrap().1.trim().to_string())
                .collect()
        })
        .unwrap_or_default();

    *wifi_list = profiles
        .iter()
        .filter_map(|p| {
            Command::new("netsh")
                .args(["wlan", "show", "profile", p.as_str(), "key=clear"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .ok()
                .map(|o| {
                    let output = String::from_utf8_lossy(&o.stdout);
                    let mut wifi = WiFi::default();

                    for line in output.lines() {
                        let (mut key, mut value) = line.split_once(':').unwrap_or_default();
                        key = key.trim();
                        value = value.trim();

                        if key == "SSID name" {
                            wifi.name = value[1..value.len() - 1].to_string();
                        }

                        if key == "Key Content" {
                            wifi.password = value.to_string();
                        }

                        if key == "Network broadcast" {
                            wifi.hidden = value.contains("Connect even");
                        }

                        if key == "Authentication" {
                            if value.contains("WPA") && value.contains("Personal") {
                                wifi.conn_type = ConnectionType::WPA;
                            } else if value.contains("WPA") && !value.contains("Personal") {
                                wifi.conn_type = ConnectionType::WPA2EAP;
                            } else if value.contains("WEB") {
                                wifi.conn_type = ConnectionType::WEP;
                            } else {
                                wifi.conn_type = ConnectionType::None;
                            }
                        }
                    }

                    wifi
                })
        })
        .collect();
}
