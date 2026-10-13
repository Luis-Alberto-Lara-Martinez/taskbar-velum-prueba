use tray_icon::Icon;

pub fn create_tray_icon() -> Icon {
    let img = image::load_from_memory(include_bytes!("../assets/logo.png"))
        .expect("Failed to open icon file")
        .into_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}
