use freya::prelude::*;

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    body: BodyTheme {
        background: cow_borrowed!("white"),
        color: cow_borrowed!("black"),
        padding: cow_borrowed!("none"),
    },
    slider: SliderTheme {
        background: cow_borrowed!("rgb(210, 210, 210)"),
        thumb_background: cow_borrowed!("rgb(210, 210, 210)"),
        thumb_inner_background: cow_borrowed!("rgb(103, 80, 164)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
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
    input: InputTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        width: cow_borrowed!("150"),
        margin: cow_borrowed!("4"),
        corner_radius: cow_borrowed!("10"),
    },
    switch: SwitchTheme {
        background: cow_borrowed!("rgb(121, 116, 126)"),
        thumb_background: cow_borrowed!("rgb(231, 224, 236)"),
        enabled_background: cow_borrowed!("rgb(103, 80, 164)"),
        enabled_thumb_background: cow_borrowed!("rgb(234, 221, 255)"),
        focus_border_fill: cow_borrowed!("rgb(180, 180, 180)"),
        enabled_focus_border_fill: cow_borrowed!("rgb(180, 180, 180)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cow_borrowed!("transparent"),
        thumb_background: cow_borrowed!("rgb(100, 100, 100, 0.30)"),
        hover_thumb_background: cow_borrowed!("rgb(160, 160, 160, 0.50)"),
        active_thumb_background: cow_borrowed!("rgb(140, 140, 140, 0.50)"),
        size: cow_borrowed!("10"),
    },
    scroll_view: ScrollViewTheme {
        height: cow_borrowed!("fill"),
        width: cow_borrowed!("fill"),
        padding: cow_borrowed!("0"),
    },
    tooltip: TooltipTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        color: cow_borrowed!("rgb(25,25,25)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cow_borrowed!("white"),
        background_button: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cow_borrowed!("white"),
        select_background: cow_borrowed!("rgb(240, 240, 240)"),
        hover_background: cow_borrowed!("rgb(220, 220, 220)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    accordion: AccordionTheme {
        color: cow_borrowed!("black"),
        background: cow_borrowed!("rgb(245, 245, 245)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    loader: LoaderTheme {
        primary_color: cow_borrowed!("rgb(50, 50, 50)"),
    },
    link: LinkTheme {
        highlight_color: cow_borrowed!("rgb(43,106,208)"),
    },
    progress_bar: ProgressBarTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(210, 210, 210)"),
        progress_background: cow_borrowed!("rgb(103, 80, 164)"),
        width: cow_borrowed!("fill"),
        height: cow_borrowed!("20"),
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cow_borrowed!("black"),
        },
        background: cow_borrowed!("white"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
        row_background: cow_borrowed!("transparent"),
        alternate_row_background: cow_borrowed!("rgb(240, 240, 240)"),
        divider_fill: cow_borrowed!("rgb(200, 200, 200)"),
        height: cow_borrowed!("auto"),
        corner_radius: cow_borrowed!("6"),
        shadow: cow_borrowed!("0 2 15 5 rgb(35, 35, 35, 70)"),
    },
    canvas: CanvasTheme {
        width: cow_borrowed!("300"),
        height: cow_borrowed!("150"),
        background: cow_borrowed!("white"),
    },
    graph: GraphTheme {
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("100%"),
    },
    network_image: NetworkImageTheme {
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("100%"),
    },
    icon: IconTheme {
        width: cow_borrowed!("10"),
        height: cow_borrowed!("10"),
        margin: cow_borrowed!("none"),
    },
    sidebar: SidebarTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    sidebar_item: SidebarItemTheme {
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("rgb(230, 230, 230)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    tile: TileTheme {
        padding: cow_borrowed!("4 6"),
    },
    radio: RadioTheme {
        unselected_fill: cow_borrowed!("rgb(35, 35, 35)"),
        selected_fill: cow_borrowed!("rgb(103, 80, 164)"),
    },
    checkbox: CheckboxTheme {
        unselected_fill: cow_borrowed!("rgb(80, 80, 80)"),
        selected_fill: cow_borrowed!("rgb(103, 80, 164)"),
        selected_icon_fill: cow_borrowed!("white"),
    },
    menu_item: MenuItemTheme {
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        corner_radius: cow_borrowed!("8"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    menu_container: MenuContainerTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        padding: cow_borrowed!("4"),
        shadow: cow_borrowed!("0 2 5 2 rgb(0, 0, 0, 0.1)"),
    },
    snackbar: SnackBarTheme {
        background: cow_borrowed!("rgb(235, 235, 235)"),
        color: cow_borrowed!("rgb(103, 80, 164)"),
    },
    popup: PopupTheme {
        background: cow_borrowed!("white"),
        color: cow_borrowed!("black"),
        cross_fill: cow_borrowed!("rgb(40, 40, 40)"),
        width: cow_borrowed!("350"),
        height: cow_borrowed!("200"),
    },
};

pub const DARK_THEME: Theme = Theme {
    name: "dark",
    body: BodyTheme {
        background: cow_borrowed!("rgb(25, 25, 25)"),
        color: cow_borrowed!("white"),
        padding: self::LIGHT_THEME.body.padding,
    },
    slider: SliderTheme {
        background: cow_borrowed!("rgb(60, 60, 60)"),
        thumb_background: cow_borrowed!("rgb(60, 60, 60)"),
        thumb_inner_background: cow_borrowed!("rgb(255, 95, 0)"),
        border_fill: cow_borrowed!("rgb(110, 110, 110)"),
    },
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
    input: InputTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
        width: self::LIGHT_THEME.input.width,
        margin: self::LIGHT_THEME.input.margin,
        corner_radius: self::LIGHT_THEME.input.corner_radius,
    },
    switch: SwitchTheme {
        background: cow_borrowed!("rgb(60, 60, 60)"),
        thumb_background: cow_borrowed!("rgb(200, 200, 200)"),
        enabled_background: cow_borrowed!("rgb(255, 95, 0)"),
        enabled_thumb_background: cow_borrowed!("rgb(234, 221, 255)"),
        focus_border_fill: cow_borrowed!("rgb(110, 110, 110)"),
        enabled_focus_border_fill: cow_borrowed!("rgb(170, 170, 170)"),
    },
    scroll_bar: ScrollBarTheme {
        background: self::LIGHT_THEME.scroll_bar.background,
        thumb_background: cow_borrowed!("rgb(100, 100, 100, 0.30)"),
        hover_thumb_background: cow_borrowed!("rgb(160, 160, 160, 0.50)"),
        active_thumb_background: cow_borrowed!("rgb(140, 140, 140, 0.50)"),
        size: self::LIGHT_THEME.scroll_bar.size,
    },
    scroll_view: ScrollViewTheme {
        height: self::LIGHT_THEME.scroll_view.height,
        width: self::LIGHT_THEME.scroll_view.width,
        padding: self::LIGHT_THEME.scroll_view.padding,
    },
    tooltip: TooltipTheme {
        background: cow_borrowed!("rgb(35,35,35)"),
        color: cow_borrowed!("rgb(240,240,240)"),
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cow_borrowed!("rgb(25, 25, 25)"),
        background_button: cow_borrowed!("rgb(35, 35, 35)"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
        arrow_fill: cow_borrowed!("rgb(150, 150, 150)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        select_background: cow_borrowed!("rgb(80, 80, 80)"),
        hover_background: cow_borrowed!("rgb(55, 55, 55)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    accordion: AccordionTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(60, 60, 60)"),
        border_fill: cow_borrowed!("rgb(80, 80, 80)"),
    },
    loader: LoaderTheme {
        primary_color: cow_borrowed!("rgb(150, 150, 150)"),
    },
    link: LinkTheme {
        highlight_color: cow_borrowed!("rgb(43,106,208)"),
    },
    progress_bar: ProgressBarTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(60, 60, 60)"),
        progress_background: cow_borrowed!("rgb(255, 95, 0)"),
        width: self::LIGHT_THEME.progress_bar.width,
        height: self::LIGHT_THEME.progress_bar.height,
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
        background: cow_borrowed!("rgb(25, 25, 25)"),
        arrow_fill: cow_borrowed!("rgb(150, 150, 150)"),
        row_background: cow_borrowed!("transparent"),
        alternate_row_background: cow_borrowed!("rgb(50, 50, 50)"),
        divider_fill: cow_borrowed!("rgb(100, 100, 100)"),
        height: self::LIGHT_THEME.table.height,
        corner_radius: self::LIGHT_THEME.table.corner_radius,
        shadow: self::LIGHT_THEME.table.shadow,
    },
    canvas: CanvasTheme {
        width: self::LIGHT_THEME.canvas.width,
        height: self::LIGHT_THEME.canvas.height,
        background: cow_borrowed!("white"),
    },
    graph: GraphTheme {
        width: self::LIGHT_THEME.graph.width,
        height: self::LIGHT_THEME.graph.height,
    },
    network_image: NetworkImageTheme {
        width: self::LIGHT_THEME.network_image.width,
        height: self::LIGHT_THEME.network_image.height,
    },
    icon: IconTheme {
        width: self::LIGHT_THEME.icon.width,
        height: self::LIGHT_THEME.icon.height,
        margin: self::LIGHT_THEME.icon.margin,
    },
    sidebar: SidebarTheme {
        background: cow_borrowed!("rgb(20, 20, 20)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    sidebar_item: SidebarItemTheme {
        background: cow_borrowed!("transparent"),
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    tile: TileTheme {
        padding: self::LIGHT_THEME.tile.padding,
    },
    radio: RadioTheme {
        unselected_fill: cow_borrowed!("rgb(245, 245, 245)"),
        selected_fill: cow_borrowed!("rgbrgb(103, 80, 164)"),
    },
    checkbox: CheckboxTheme {
        unselected_fill: cow_borrowed!("rgb(245, 245, 245)"),
        selected_fill: cow_borrowed!("rgb(103, 80, 164)"),
        selected_icon_fill: cow_borrowed!("white"),
    },
    menu_item: MenuItemTheme {
        hover_background: cow_borrowed!("rgb(45, 45, 45)"),
        corner_radius: self::LIGHT_THEME.menu_item.corner_radius,
        font_theme: FontTheme {
            color: cow_borrowed!("white"),
        },
    },
    menu_container: MenuContainerTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        padding: self::LIGHT_THEME.menu_container.padding,
        shadow: self::LIGHT_THEME.menu_container.shadow,
    },
    snackbar: SnackBarTheme {
        background: cow_borrowed!("rgb(35, 35, 35)"),
        color: cow_borrowed!("white"),
    },
    popup: PopupTheme {
        background: cow_borrowed!("rgb(25, 25, 25)"),
        color: cow_borrowed!("white"),
        cross_fill: cow_borrowed!("rgb(150, 150, 150)"),
        width: self::LIGHT_THEME.popup.width,
        height: self::LIGHT_THEME.popup.height,
    },
};
