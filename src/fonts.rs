pub struct Font {
    pub name: &'static str,
    pub filename: &'static str,
}

impl Font {
    pub const fn new(name: &'static str, filename: &'static str) -> Self {
        Self { name, filename }
    }

    pub fn load(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(format!("C:\\Windows\\Fonts\\{}", self.filename))
    }
}

pub const SEGOE_UI_VARIABLE: Font = Font::new("Segoe UI Variable", "SegUIVar.ttf");
pub const SEGOE_FLUENT_ICONS: Font = Font::new("Segoe Fluent Icons", "SegoeIcons.ttf");
