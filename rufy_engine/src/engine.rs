extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread;

use rufy_core::{
    Sender,
    Receiver,
};

pub struct Engine {
    recipient: Option<Receiver>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            recipient: None,
        }
    }

    pub fn init(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rufy", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn init_signal(&mut self) -> rufy_core::Sender {
        let (sx, mut rx) = rufy_core::create_channel();
        self.recipient = Some(rx);
        sx
    }

    pub async fn listen_signal(self) {
        match self.recipient {
            None => println!("listen signal is none"),
            Some(mut rc) => {
                tokio::spawn(async move {
                    while let Some(msg) = rc.recv().await {
                        println!("GOT = {}", msg);
                    }
                });
            },
        }
    }
}