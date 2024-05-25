fn main() -> std::io::Result<()> {
    winres::WindowsResource::new()
        .set_icon("./icons/icon.ico")
        .compile()
}
