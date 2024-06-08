use freya::prelude::*;

use crate::{icons, MAIN_WINDOW_HWND};

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    button: ButtonTheme {
        background: cow_borrowed!("rgb(250, 250, 250, 0.75)"),
        hover_background: cow_borrowed!("rgb(210, 210, 210, 0.3)"),
        font_theme: FontTheme {
            color: cow_borrowed!("#1B1B1B"),
        },
        border_fill: cow_borrowed!("1 solid rgb(160, 160, 160, 0.25)"),
        focus_border_fill: cow_borrowed!("2 solid rgb(26, 26, 26)"),
        shadow: cow_borrowed!("0 4 5 0 rgb(0, 0, 0, 0.1)"),
        padding: cow_borrowed!("8"),
        margin: cow_borrowed!("0"),
        corner_radius: cow_borrowed!("5"),
        width: cow_borrowed!("auto"),
        height: cow_borrowed!("auto"),
    },
    ..freya_hooks::LIGHT_THEME
};

pub const DARK_THEME: Theme = Theme {
    name: "dark",

    button: ButtonTheme {
        background: cow_borrowed!("rgb(100, 100, 100, 0.15)"),
        hover_background: cow_borrowed!("rgb(160, 160, 160, 0.15)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("1 solid rgb(0, 0, 0, 0.3)"),
        focus_border_fill: cow_borrowed!("2 solid rgb(255, 255, 255)"),
        shadow: cow_borrowed!("0 4 5 0 rgb(0, 0, 0, 0.1)"),
        padding: self::LIGHT_THEME.button.padding,
        margin: self::LIGHT_THEME.button.margin,
        corner_radius: self::LIGHT_THEME.button.corner_radius,
        width: self::LIGHT_THEME.button.width,
        height: self::LIGHT_THEME.button.height,
    },
    ..freya_hooks::DARK_THEME
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    System,
    Dark,
    Light,
}

impl ThemeVariant {
    pub fn next(self) -> Self {
        let system_theme = use_preferred_theme();
        let is_dark = matches!(*system_theme.read(), PreferredTheme::Dark);

        if is_dark {
            match self {
                Self::System => Self::Light,
                Self::Light => Self::Dark,
                Self::Dark => Self::System,
            }
        } else {
            match self {
                Self::System => Self::Dark,
                Self::Dark => Self::Light,
                Self::Light => Self::System,
            }
        }
    }

    pub fn next_icon(self) -> &'static str {
        self.next().icon()
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ThemeVariant::System => icons::CONTRAST,
            ThemeVariant::Dark => icons::MOON,
            ThemeVariant::Light => icons::SUN,
        }
    }

    pub fn apply_mica(self) {
        let _ = match self {
            ThemeVariant::System => match *use_preferred_theme().read() {
                PreferredTheme::Dark => {
                    window_vibrancy::apply_mica(unsafe { MAIN_WINDOW_HWND }, Some(true))
                }
                PreferredTheme::Light => {
                    window_vibrancy::apply_mica(unsafe { MAIN_WINDOW_HWND }, Some(false))
                }
            },
            ThemeVariant::Dark => {
                window_vibrancy::apply_mica(unsafe { MAIN_WINDOW_HWND }, Some(true))
            }
            ThemeVariant::Light => {
                window_vibrancy::apply_mica(unsafe { MAIN_WINDOW_HWND }, Some(false))
            }
        };
    }
}

impl From<ThemeVariant> for Theme {
    fn from(value: ThemeVariant) -> Self {
        match value {
            ThemeVariant::System => match *use_preferred_theme().read() {
                PreferredTheme::Dark => DARK_THEME,
                PreferredTheme::Light => LIGHT_THEME,
            },
            ThemeVariant::Dark => DARK_THEME,
            ThemeVariant::Light => LIGHT_THEME,
        }
    }
}

pub fn use_init_theme() -> Signal<Theme> {
    let theme = *use_preferred_theme().read();
    use_context_provider(|| {
        Signal::new(match theme {
            PreferredTheme::Dark => DARK_THEME,
            PreferredTheme::Light => LIGHT_THEME,
        })
    })
}
