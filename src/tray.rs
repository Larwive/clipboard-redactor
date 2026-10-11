use tray_icon;

use tray_icon::Icon;

fn make_icon() -> Icon {
    let (w, h) = (32u32, 32u32);
    let rgba: Vec<u8> = (0..w * h).flat_map(|_| [220, 40, 40, 255]).collect();
    Icon::from_rgba(rgba, w, h).expect("valid icon data")
}

pub fn create_tray_icon() -> tray_icon::TrayIcon{
    use tray_icon::{TrayIconBuilder, menu::Menu,Icon};
    let icon = make_icon();
    let tray_menu = Menu::new();
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();
    return tray_icon;
}