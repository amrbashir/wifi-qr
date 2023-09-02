fn main() -> std::io::Result<()> {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./.github/icon.ico");
    res.compile()
}
