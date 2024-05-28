use std::path::Path;

const RESOURCES: &str = r#"1 ICON "icons/icon.ico""#;

fn main() -> std::io::Result<()> {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not defined");
    let path = Path::new(&out_dir).join("app.rc");
    std::fs::write(&path, RESOURCES)?;
    embed_resource::compile(path, embed_resource::NONE);
    Ok(())
}
