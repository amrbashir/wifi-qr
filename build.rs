fn main() -> std::io::Result<()> {
    winres::WindowsResource::new()
        .set_icon("./.github/icon.ico")
        .compile()
}
