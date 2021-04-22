use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude, Cursor, Read};

const HELP_TEXT: &'static str = r#"
OBI Viewer
"#;

use obi::Image;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    match &args[..] {
        [_, path] => {
            eprintln!("{}", path);
            let image_src = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(path);
            if let Ok(mut image) = image_src {
                let mut buf = Vec::new();
                image.read_to_end(&mut buf);
                let img = Image::decode(&mut (Cursor::new(buf))).unwrap();

                let window = video_subsystem
                    .window("OBI Viewer", img.width(), img.height())
                    .position_centered()
                    .opengl()
                    .build()
                    .map_err(|e| e.to_string())
                    .unwrap();

                let mut canvas = window
                    .into_canvas()
                    .build()
                    .map_err(|e| e.to_string())
                    .unwrap();

                let mut event_pump = sdl_context.event_pump().unwrap();

                'running: loop {
                    for event in event_pump.poll_iter() {
                        match event {
                            Event::Quit { .. }
                            | Event::KeyDown {
                                keycode: Some(Keycode::Escape),
                                ..
                            } => break 'running,
                            _ => {}
                        }
                    }

                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                    canvas.set_draw_color(Color::RGB(255, 255, 255));

                    for x in 0..img.width() {
                        for y in 0..img.height() {
                            if img.data[img.index(x, y).unwrap()] {
                                canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1));
                            }
                        }
                    }

                    canvas.present();
                }
            }
        }
        _ => {
            eprintln!("requires and argument!");
        }
    }
}
