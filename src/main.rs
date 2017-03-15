extern crate gtk;
extern crate gdk;
extern crate gdk_sys;
extern crate gdk_pixbuf;
extern crate cairo;
extern crate screenshot;
extern crate image;
extern crate epoxy;
extern crate gl;
extern crate shared_library;

use gtk::prelude::*;
use screenshot::get_screenshot;
use self::gl::types::*;
use std::cell::{RefCell, Cell};
use std::f64::consts::PI;
use std::ffi::CStr;
use std::mem;
use std::process::Command;
use std::ptr;
use std::rc::Rc;

use gtk::{Button, Window, WindowType};

fn main() {
    // TODO: Use env SCREEN or whatever
    let screenshot = get_screenshot(0).unwrap();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Popup);
    window.set_title("Lockscreen");
    window.set_type_hint(gdk::WindowTypeHint::Dialog);

    window.set_decorated(false);
    window.set_app_paintable(true);

    // Get primary screen geometry
    let screen = window.get_screen().unwrap();
    let monitor_id = screen.get_primary_monitor();
    let monitor = screen.get_monitor_geometry(monitor_id);

    window.move_(0, 0);
    window.set_size_request(screen.get_width(), screen.get_height());

    // Set up styles
    let style_context = window.get_style_context().unwrap();
    let css_provider = gtk::CssProvider::new();
    let _ = css_provider.load_from_data("* { background-color: rgba(27, 29, 31, 0.8); }");
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Button
    let button = Button::new_with_label("Unlock!");
    button.set_size_request(80,32);

    // Background
    let image_buffer = gdk_pixbuf::Pixbuf::new_from_file_at_scale("lockscreen2.png", monitor.width, monitor.height, false).unwrap();
    let image = gtk::Image::new_from_pixbuf(Some(&image_buffer));

    // Add background to window
    let container = gtk::Fixed::new();
    container.put(&image, 0,0);
    container.put(&button, monitor.width / 2, monitor.height / 2);
    window.add(&container);
    window.show_all();


    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    button.connect_clicked(|_| {
        println!("Clicked!");
        gtk::main_quit();
    });

    gtk::main();
}
