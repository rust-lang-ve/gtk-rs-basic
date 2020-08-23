use gtk::*;

pub struct Header {
    pub container: HeaderBar,
}

impl Header {
    pub fn new() -> Self {
        let container = HeaderBar::new();

        container.set_title(Some("Application Name"));
        container.set_show_close_button(true);

        Self { container }
    }
}
