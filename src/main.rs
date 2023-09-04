#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui::{self, vec2, Id, Margin, Modifiers, Rounding, Sense},
    emath::Align,
    epaint::{Color32, Stroke},
    IconData,
};
use raw_window_handle::HasRawWindowHandle;

use route::Route;
use state::State;

mod fonts;
mod pages;
mod route;
mod state;
mod widgets;
mod wifi;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(300.0, 350.0)),
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        transparent: true,
        decorated: false,
        centered: true,
        resizable: true,
        icon_data: Some(
            IconData::try_from_png_bytes(include_bytes!("../.github/icon.png")).unwrap(),
        ),
        ..Default::default()
    };
    eframe::run_native("WiFi QR", options, Box::new(|cc| Box::new(App::new(cc))))
}

#[derive(Default)]
pub struct App {
    state: State,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::apply_effects(cc, cc.egui_ctx.style().visuals.dark_mode);
        let _ = Self::setup_fonts(&cc.egui_ctx);
        Self {
            ..Default::default()
        }
    }

    fn apply_effects(handle: &impl HasRawWindowHandle, dark: bool) {
        let _ = window_vibrancy::apply_mica(handle, Some(dark));
        let _ = window_shadows::set_shadow(handle, true);
    }

    fn setup_fonts(ctx: &egui::Context) -> std::io::Result<()> {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            fonts::SEGOE_UI_VARIABLE.name.to_owned(),
            egui::FontData::from_owned(fonts::SEGOE_UI_VARIABLE.load()?),
        );
        fonts.font_data.insert(
            fonts::SEGOE_FLUENT_ICONS.name.to_owned(),
            egui::FontData::from_owned(fonts::SEGOE_FLUENT_ICONS.load()?),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, fonts::SEGOE_FLUENT_ICONS.name.to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, fonts::SEGOE_UI_VARIABLE.name.to_owned());

        ctx.set_fonts(fonts);

        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                // window effects
                let dark_mode = ui.visuals().dark_mode;
                if dark_mode != self.state.dark_mode {
                    Self::apply_effects(frame, dark_mode);
                    self.state.dark_mode = dark_mode;
                }

                // general styles
                ui.style_mut().visuals.widgets.noninteractive.fg_stroke = Stroke::new(
                    1.0,
                    if dark_mode {
                        Color32::WHITE
                    } else {
                        Color32::from_rgba_unmultiplied(27, 27, 27, 255)
                    },
                );

                // keyboard navigation
                ctx.input(|input| {
                    let alt = input.modifiers.contains(Modifiers::ALT);
                    if alt {
                        if input.key_down(egui::Key::ArrowLeft) {
                            Route::back(&mut self.state);
                        } else if input.key_down(egui::Key::ArrowRight) {
                            Route::forward(&mut self.state);
                        }
                    }
                });

                // TODOs:
                // 1. adjust titlebar buttons styling and sizes
                egui::Frame::none().show(ui, |ui| {
                    // render titlebar
                    let tb_height = 32.0;
                    let tb_padding = 8.0;
                    let tb_rect = {
                        let mut rect = ui.max_rect();
                        rect.max.y = rect.min.y + 32.0;
                        rect
                    };

                    let tb_response = ui.interact(tb_rect, Id::new("title_bar"), Sense::click());

                    if tb_response.double_clicked() {
                        frame.set_maximized(!frame.info().window_info.maximized);
                    } else if tb_response.is_pointer_button_down_on() {
                        frame.drag_window();
                    }

                    ui.allocate_ui_at_rect(tb_rect, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.add_space(tb_padding);
                                ui.horizontal_centered(|ui| {
                                    ui.add_space(tb_padding);
                                    if ui
                                        .add_enabled(
                                            self.state.previous_route.is_some(),
                                            |ui: &mut egui::Ui| {
                                                ui.add_sized(
                                                    [0.0, tb_height - tb_padding],
                                                    // back
                                                    widgets::Button::new("")
                                                        .flat(true)
                                                        .padding(vec2(12.0, 12.0)),
                                                )
                                            },
                                        )
                                        .clicked()
                                    {
                                        Route::back(&mut self.state)
                                    }

                                    ui.label("WiFi QR");
                                });
                            });

                            ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                                ui.horizontal_centered(|ui| {
                                    let style = ui.style_mut();
                                    let default_item_spacing = style.spacing.item_spacing;
                                    style.spacing.item_spacing = vec2(0.0, 0.0);

                                    if ui
                                        .add(
                                            // close
                                            widgets::Button::new("")
                                                .rounding(Rounding::none())
                                                .flat(true),
                                        )
                                        .clicked()
                                    {
                                        frame.close();
                                    }
                                    let maximized = frame.info().window_info.maximized;
                                    if ui
                                        .add(
                                            // maximize
                                            widgets::Button::new(if maximized {
                                                ""
                                            } else {
                                                ""
                                            })
                                            .rounding(Rounding::none())
                                            .flat(true),
                                        )
                                        .clicked()
                                    {
                                        frame.set_maximized(!maximized);
                                    }
                                    if ui
                                        .add(
                                            // minimize
                                            widgets::Button::new("")
                                                .rounding(Rounding::none())
                                                .flat(true),
                                        )
                                        .clicked()
                                    {
                                        frame.set_minimized(true);
                                    }

                                    ui.add_space(default_item_spacing.x);
                                    ui.style_mut().spacing.item_spacing = default_item_spacing;

                                    if ui
                                        .add(
                                            // dark mode toggle
                                            widgets::Button::new(if dark_mode {
                                                ""
                                            } else {
                                                ""
                                            })
                                            .rounding(Rounding::none())
                                            .flat(true),
                                        )
                                        .clicked()
                                    {
                                        ctx.set_visuals(if dark_mode {
                                            egui::Visuals::light()
                                        } else {
                                            egui::Visuals::dark()
                                        });
                                    }

                                    if self.state.selected_route == Route::WifiList
                                        && ui
                                            .add(
                                                // refresh
                                                widgets::Button::new("")
                                                    .rounding(Rounding::none())
                                                    .flat(true),
                                            )
                                            .clicked()
                                    {
                                        self.state.requested_refresh = true;
                                    }

                                    ui.add_space(ui.available_width());
                                })
                            });
                        });
                    });

                    // Render route
                    egui::Frame::none()
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            self.state
                                .selected_route
                                .clone()
                                .render(&mut self.state, ui);
                        });
                })
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
}
