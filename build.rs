#[cfg(target_os = "windows")]
fn embed_icon() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().expect("Failed to compile Windows Resources");
}

pub fn main() {
    #[cfg(target_os = "windows")]
    embed_icon();
}
