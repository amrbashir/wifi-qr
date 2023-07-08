pub struct Font {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

impl Font {
    pub const fn new(name: &'static str, bytes: &'static [u8]) -> Self {
        Self { name, bytes }
    }
}

pub const SEGOE_UI_VARIABLE: Font =
    Font::new("Segoe UI Variable", include_bytes!("./SegUIVar.ttf"));
pub const SEGOE_FLUENT_ICONS: Font =
    Font::new("Segoe Fluent Icons", include_bytes!("./SegoeIcons.ttf"));
