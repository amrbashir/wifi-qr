use eframe::{
    egui::{Response, Sense, TextStyle, Ui, Vec2, Widget, WidgetInfo, WidgetText, WidgetType},
    emath::NumExt,
    epaint::{pos2, vec2, Color32, Rounding, Stroke},
};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct Button {
    text: WidgetText,
    subtitle: WidgetText,
    wrap: Option<bool>,
    /// None means default for interact
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    sense: Sense,
    small: bool,
    flat: bool,
    frame: Option<bool>,
    min_size: Vec2,
    rounding: Option<Rounding>,
    icon: WidgetText,
    padding: Option<Vec2>,
}

#[allow(unused)]
impl Button {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            subtitle: Default::default(),
            wrap: None,
            fill: None,
            stroke: None,
            sense: Sense::click(),
            small: false,
            flat: false,
            frame: None,
            min_size: Vec2::ZERO,
            rounding: None,
            icon: Default::default(),
            padding: None,
        }
    }

    pub fn icon_and_text(icon_text: impl Into<WidgetText>, text: impl Into<WidgetText>) -> Self {
        Self {
            icon: icon_text.into(),
            ..Self::new(text)
        }
    }

    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self.frame = Some(true);
        self
    }

    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self.frame = Some(true);
        self
    }

    pub fn small(mut self) -> Self {
        self.text = self.text.text_style(TextStyle::Body);
        self.small = true;
        self
    }

    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = Some(frame);
        self
    }

    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    pub fn rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.rounding = Some(rounding.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<WidgetText>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn flat(mut self, flat: bool) -> Self {
        self.flat = flat;
        self
    }

    pub fn padding(mut self, padding: Vec2) -> Self {
        self.padding = Some(padding);
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let Button {
            text,
            subtitle,
            wrap,
            fill,
            stroke,
            sense,
            small,
            frame,
            min_size,
            rounding,
            icon,
            flat,
            padding,
        } = self;

        let frame = frame.unwrap_or_else(|| ui.visuals().button_frame);

        let button_padding = padding.unwrap_or_else(|| {
            if small {
                vec2(8.0, 0.0)
            } else {
                vec2(16.0, 16.0)
            }
        });

        let mut text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        let icon = (!icon.is_empty())
            .then(|| icon.into_galley(ui, wrap, text_wrap_width, TextStyle::Heading));
        if let Some(icon) = &icon {
            text_wrap_width -= icon.size().x + ui.spacing().icon_spacing;
        }
        let text = text.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);
        let subtitle = (!subtitle.is_empty())
            .then(|| subtitle.into_galley(ui, wrap, text_wrap_width, TextStyle::Small));

        let text_size = text.size();
        let mut desired_size = text_size;
        if let Some(subtitle) = &subtitle {
            desired_size.x = desired_size.x.at_least(subtitle.size().x);
            desired_size.y += subtitle.size().y + ui.spacing().item_spacing.y;
        }
        if let Some(icon) = &icon {
            desired_size.x += icon.size().x + ui.spacing().icon_spacing;
            desired_size.y = desired_size.y.max(icon.size().y);
        }

        desired_size += 2.0 * button_padding;
        if !small {
            desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        }
        desired_size = desired_size.at_least(min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        if ui.is_rect_visible(rect) {
            let dark_mode = ui.style().visuals.dark_mode;
            let visuals = {
                let style = ui.style_mut();
                if dark_mode {
                    style.visuals.widgets.active.weak_bg_fill =
                        Color32::from_rgba_unmultiplied(100, 100, 100, 5);
                    style.visuals.widgets.hovered.weak_bg_fill = if flat {
                        Color32::from_rgba_unmultiplied(160, 160, 160, 2)
                    } else {
                        Color32::from_rgba_unmultiplied(160, 160, 160, 5)
                    };
                    style.visuals.widgets.inactive.weak_bg_fill = if flat {
                        Color32::TRANSPARENT
                    } else {
                        Color32::from_rgba_unmultiplied(125, 125, 125, 5)
                    };
                    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::WHITE);
                    style.visuals.widgets.active.fg_stroke = if flat {
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 70))
                    } else {
                        Stroke::new(1.0, Color32::WHITE)
                    };
                } else {
                    style.visuals.widgets.active.weak_bg_fill = if flat {
                        Color32::from_rgba_unmultiplied(27, 27, 27, 10)
                    } else {
                        Color32::TRANSPARENT
                    };
                    style.visuals.widgets.hovered.weak_bg_fill = if flat {
                        Color32::from_rgba_unmultiplied(27, 27, 27, 14)
                    } else {
                        Color32::from_rgba_unmultiplied(50, 50, 50, 1)
                    };
                    style.visuals.widgets.inactive.weak_bg_fill = if flat {
                        Color32::TRANSPARENT
                    } else {
                        Color32::from_rgba_unmultiplied(150, 150, 150, 3)
                    };
                    style.visuals.widgets.inactive.fg_stroke =
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(27, 27, 27, 255));
                    style.visuals.widgets.active.fg_stroke = if flat {
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(27, 27, 27, 160))
                    } else {
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(27, 27, 27, 255))
                    };
                }
                *style.interact(&response)
            };

            if frame {
                let fill = fill.unwrap_or(visuals.weak_bg_fill);
                let stroke = if !flat {
                    stroke.unwrap_or_else(|| {
                        if dark_mode {
                            Stroke::new(0.2, Color32::from_rgba_unmultiplied(0, 0, 0, 5))
                        } else {
                            Stroke::new(0.2, Color32::from_rgba_unmultiplied(27, 27, 27, 80))
                        }
                    })
                } else {
                    Stroke::NONE
                };
                let rounding = rounding.unwrap_or_else(|| Rounding::same(4.0));
                ui.painter().rect(rect, rounding, fill, stroke);
            }

            let mut text_pos = if let Some(icon) = &icon {
                let icon_spacing = ui.spacing().icon_spacing;
                pos2(
                    rect.min.x + button_padding.x + icon.size().x + icon_spacing * 2.0,
                    rect.center().y - 0.5 * text_size.y,
                )
            } else {
                ui.layout()
                    .align_size_within_rect(text_size, rect.shrink2(button_padding))
                    .min
            };
            if let Some(subtitle) = &subtitle {
                text_pos = pos2(
                    text_pos.x,
                    rect.center().y - 0.5 * (text_size + subtitle.size()).y,
                )
            }

            text.paint_with_visuals(ui.painter(), text_pos, &visuals);
            if let Some(subtitle) = subtitle {
                let subtitle_pos = text_pos + vec2(0.0, text_size.y + ui.spacing().item_spacing.y);
                subtitle.paint_with_visuals(ui.painter(), subtitle_pos, &visuals);
            }

            if let Some(icon) = icon {
                let icon_pos = pos2(
                    rect.min.x + button_padding.x,
                    rect.center().y - 0.5 - (icon.size().y / 2.0),
                );
                icon.paint_with_visuals(ui.painter(), icon_pos, &visuals);
            }
        }

        response
    }
}
