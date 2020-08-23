/// Ported and modified from: https://github.com/mmstick/gtkrs-tutorials/blob/master/demos/chapter_01/src/main.rs
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{init, main as gtk_main, Application};
use std::process;

mod app;
mod header;

fn main() {
    if init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    if let Ok(app) = Application::new(
        Some("com.github.rust-lang-ve.gtk-basic"),
        Default::default(),
    ) {
        app.window.show_all();

        gtk_main();
    }
}
