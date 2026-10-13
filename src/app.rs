use ::tray_icon::TrayIcon;
use ::tray_icon::menu::{MenuEvent, MenuId};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct App {
    _tray: TrayIcon,
    salir_id: MenuId,
}

impl App {
    pub fn new(tray: TrayIcon, salir_id: MenuId) -> Self {
        Self {
            _tray: tray,
            salir_id,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            if menu_event.id == self.salir_id {
                event_loop.exit();
            }
        }
    }
}
