#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;

use eframe::{
    egui::{self, Layout, Margin, Modifiers, ViewportCommand},
    epaint::{Color32, Stroke},
    icon_data,
};
use raw_window_handle::HasWindowHandle;

use route::Route;
use state::State;
use windows_sys::Win32::{Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS};

mod fonts;
mod pages;
mod route;
mod state;
mod widgets;
mod wifi;

const WINDOW_ICON: &[u8] = include_bytes!("../icons/icon.png");

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        window_builder: Some(Box::new(|builder| {
            builder
                .with_inner_size([600.0, 600.0])
                .with_min_inner_size([300.0, 350.0])
                .with_transparent(true)
                .with_resizable(true)
                .with_icon(icon_data::from_png_bytes(WINDOW_ICON).unwrap())
        })),
        ..Default::default()
    };
    eframe::run_native(
        "WiFi QR",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(App::new(cc))
        }),
    )
}

#[derive(Default)]
pub struct App {
    state: State,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::apply_effects(cc, cc.egui_ctx.style().visuals.dark_mode);
        let _ = Self::setup_fonts(&cc.egui_ctx);

        Self::default()
    }

    fn apply_effects(handle: &impl HasWindowHandle, dark: bool) {
        let _ = window_vibrancy::apply_mica(handle, Some(dark));
        Self::apply_shadow(handle)
    }

    fn apply_shadow(handle: &impl HasWindowHandle) {
        if let raw_window_handle::RawWindowHandle::Win32(handle) =
            handle.window_handle().unwrap().as_raw()
        {
            unsafe {
                let margins = MARGINS {
                    cyTopHeight: 1,
                    cxLeftWidth: 1,
                    cxRightWidth: 1,
                    cyBottomHeight: 1,
                };
                DwmExtendFrameIntoClientArea(handle.hwnd.get(), &margins);
            }
        }
    }

    // TODO: seems like eframe fails to change icon on startup, let's workaround it for now
    fn change_icon(&mut self, ctx: &egui::Context) {
        if self.state.changed_icon_counter < 5 {
            let icon_data = Arc::new(icon_data::from_png_bytes(WINDOW_ICON).unwrap());
            ctx.send_viewport_cmd(ViewportCommand::Icon(Some(icon_data)));

            self.state.changed_icon_counter += 1;
        }
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
                self.change_icon(ctx);

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

                egui::Frame::none().show(ui, |ui| {
                    // Render route
                    egui::Frame::none()
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                // back
                                if ui
                                    .add_enabled(
                                        self.state.previous_route.is_some(),
                                        |ui: &mut egui::Ui| {
                                            ui.add(widgets::Button::new("").flat(true))
                                        },
                                    )
                                    .clicked()
                                {
                                    Route::back(&mut self.state)
                                }

                                // refresh
                                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                    // dark mode toggle
                                    if ui
                                        .add(
                                            widgets::Button::new(if dark_mode {
                                                ""
                                            } else {
                                                ""
                                            })
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
                                        && ui.add(widgets::Button::new("").flat(true)).clicked()
                                    {
                                        self.state.requested_refresh = true;
                                    }
                                });
                            });

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
