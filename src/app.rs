use crate::header::Header;
use gtk::Window;

pub struct App {
    pub window: Window,
    pub header: Header,
}

impl App {
    fn new() -> Self {
        let window = Window::new(WindowType::TopLevel);
        let header = Header::new();

        window.set_titlebar(&header.container);
        window.set_title("Application Name");
        window.set_wmclass("app-name", "Application Name");

        Window::set_default_icon_name("iconname");

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhitib(false)
        });

        Self { window, header }
    }
}
