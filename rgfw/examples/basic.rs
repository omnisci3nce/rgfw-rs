use events::Event;
use input::KeyCode;
use rgfw::*;

fn main() {
    let window = Window::create("Basic");

    let mut evt_count: usize = 0;
    while !window.should_close() {
        if window.is_pressed(KeyCode::Left) {
            println!("Left");
        }
        if window.is_pressed(KeyCode::Right) {
            println!("Right");
        }

        let events = window.events();

        let should_quit = events
            .iter()
            .find_map(|ev| if *ev == Event::Quit { Some(true) } else { None })
            .unwrap_or(false)
            || window.is_pressed(KeyCode::Escape);

        if should_quit {
            break;
        } else {
            if !events.is_empty() {
                evt_count += 1;
                dbg!(evt_count, events);
            }
        }

        window.swap_buffers();
    }
}
