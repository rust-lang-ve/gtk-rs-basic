/// Ported and modified from: https://github.com/mmstick/gtkrs-tutorials/blob/master/demos/chapter_01/src/main.rs
extern crate gio;
extern crate gtk;

use gtk::*;
use std::process;

mod app;
mod header;

fn main() {
    if init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    let app = app::App::new();

    app.window.show_all();

    gtk::main();
}
