use gtk::HeaderBar;

pub struct Header {
    pub container: HeaderBar,
}

impl Header {
    fn new() -> Self {
        let container = HeaderBar::new();

        container.set_title("Application Name");
        container.set_show_close_button(true);

        Self { container }
    }
}
