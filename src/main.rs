#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod tray_icon;

use crate::app::App;
use crate::tray_icon::create_tray_icon;
use ::tray_icon::TrayIconBuilder;
use ::tray_icon::menu::{Menu, MenuItem};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let menu = Menu::new();
    let salir_item = MenuItem::new("Salir", true, None);
    menu.append(&salir_item).unwrap();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Taskbar Velum")
        .with_icon(create_tray_icon())
        .build()
        .expect("Error al crear el icono en el tray");

    let mut app = App::new(tray, salir_item.id().clone());

    event_loop.run_app(&mut app).unwrap();
}
