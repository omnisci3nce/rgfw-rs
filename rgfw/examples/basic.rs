use rgfw::*;
use rgfw_sys::RGFW_quit;

fn main() {
    let window = Window::create("Basic");

    while !window.should_close() {
        if window.is_pressed(Key::Left) {
            println!("Left");
        }
        if window.is_pressed(Key::Right) {
            println!("Right");
        }

        let should_quit = window
            .check_event()
            .map(|ev| ev.type_ == RGFW_quit)
            .unwrap_or(false)
            || window.is_pressed(Key::Escape);

        if should_quit {
            break;
        }

        window.swap_buffers();
    }
}
