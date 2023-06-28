pub enum Event {
    A,
    B,
}

pub fn map_event(event: sdl2::event::Event) -> Option<Event> {
    match event {
        sdl2::event::Event::KeyDown {
            keycode: Some(sdl2::keyboard::Keycode::Z),
            ..
        } => Some(Event::A),
        _ => None,
    }
}
