use freya::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FluentButtonProps {
    pub children: Element,
    pub theme: Option<ButtonThemeWith>,
    pub enabled: Option<bool>,
    pub onclick: Option<EventHandler<Option<MouseEvent>>>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub padding: Option<String>,
    pub margin: Option<String>,
    pub corner_radius: Option<String>,
    pub direction: Option<String>,
    pub background: Option<String>,
    pub hover_background: Option<String>,
    pub color: Option<String>,
    pub disabled_color: Option<String>,
    pub cross_align: Option<String>,
    pub main_align: Option<String>,
    pub flat: Option<bool>,
}

#[allow(non_snake_case)]
pub fn FluentButton(props: FluentButtonProps) -> Element {
    let enabled = props.enabled.unwrap_or(true);

    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();

    let user_click = &props.onclick;

    let onclick = {
        to_owned![user_click];
        move |ev| {
            if enabled {
                focus.focus();
                if let Some(onclick) = &user_click {
                    onclick.call(Some(ev))
                }
            }
        }
    };

    let onmouseenter = move |_| {
        if enabled {
            platform.set_cursor(CursorIcon::Pointer);
            status.set(ButtonStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        if enabled {
            platform.set_cursor(CursorIcon::default());
            status.set(ButtonStatus::default());
        }
    };

    use_drop(move || {
        platform.set_cursor(CursorIcon::Default);
    });

    let onkeydown = {
        move |e: KeyboardEvent| {
            if enabled && focus.validate_keydown(e) {
                if let Some(onclick) = &props.onclick {
                    onclick(None)
                }
            }
        }
    };

    let theme = use_theme();
    let is_dark = theme.read().name == crate::theme::DARK_THEME.name;

    let theme = use_applied_theme!(&props.theme, button);

    let flat = props.flat.unwrap_or(false);

    let background = match *status.read() {
        ButtonStatus::Hovering if enabled => props
            .hover_background
            .as_deref()
            .unwrap_or(&theme.hover_background),
        _ if flat => "transparent",
        _ => props.background.as_deref().unwrap_or(&theme.background),
    };

    let color = if enabled {
        props.color.as_deref().unwrap_or(&theme.font_theme.color)
    } else {
        props
            .disabled_color
            .as_deref()
            .unwrap_or(if is_dark { "#78787D" } else { "#A0A0A2" })
    };

    let border = if focus.is_selected() {
        &theme.focus_border_fill
    } else if flat {
        ""
    } else {
        &theme.border_fill
    };

    let width = props.width.as_deref().unwrap_or(&theme.width);
    let height = props.height.as_deref().unwrap_or(&theme.height);
    let padding = props.padding.as_deref().unwrap_or(&theme.padding);
    let margin = props.margin.as_deref().unwrap_or(&theme.margin);
    let direction = props.direction.as_deref().unwrap_or("vertical");
    let cross_align = props.cross_align.as_deref().unwrap_or("center");
    let main_align = props.main_align.as_deref().unwrap_or("center");
    let corner_radius = props
        .corner_radius
        .as_deref()
        .unwrap_or(&theme.corner_radius);

    rsx!(
        rect {
            onclick,
            onmouseenter,
            onmouseleave,
            onkeydown,
            focus_id,
            border,
            width,
            height,
            background,
            padding,
            margin,
            corner_radius,
            color,
            direction,
            cross_align,
            main_align,
            {&props.children}
        }
    )
}
