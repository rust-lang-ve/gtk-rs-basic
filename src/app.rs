use crate::header::Header;
use gtk::*;

pub struct App {
    pub window: Window,
    pub header: Header,
}

impl App {
    pub fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("Application Name");
        window.set_wmclass("app-name", "Application Name");

        Window::set_default_icon_name("iconname");

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        Self { window, header }
    }
}
