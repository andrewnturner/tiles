use std::{thread::sleep, time::Duration};

use event::map_event;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    video::{self, Window},
    VideoSubsystem,
};

mod event;
mod game;
mod resource;
mod util;
mod world;

use crate::game::Game;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = init_canvas(video_subsystem);
    let texture_creator = canvas.texture_creator();

    let mut game = Game::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("quit");
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("escape");
                    // break 'running;
                }
                _ => {
                    if let Some(mapped_event) = map_event(event) {
                        game.process_event(mapped_event);
                    }
                }
            }
        }

        game.draw(&mut canvas, &texture_creator);
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn init_canvas(video_subsystem: VideoSubsystem) -> Canvas<Window> {
    let window = video_subsystem
        .window("tiles", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    window.into_canvas().build().unwrap()
}
