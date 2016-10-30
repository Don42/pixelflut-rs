extern crate rand;
extern crate piston_window;
extern crate image as im;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self,Sender,Receiver};
use std::{time, thread};

use piston_window::*;

type FrameBuffer = Arc<Mutex<im::ImageBuffer<im::Rgba<u8>, Vec<u8>>>>;

fn render(tx: Sender<Command>, frame_buffer: FrameBuffer) {

    let mut window: PistonWindow = WindowSettings::new("piston", [512; 2])
		.exit_on_esc(true)
        .build()
        .unwrap();
	window.set_max_fps(60);

    window.set_bench_mode(true);
    while let Some(e) = window.next() {
		let texture = Texture::from_image(&mut window.factory,
										  &frame_buffer.lock().unwrap(),
										  &TextureSettings::new()
										  ).unwrap();

        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            image(&texture, c.transform, g);
        });
    }
    tx.send(Command::Shutdown).unwrap();
}

fn paint(frame_buffer: FrameBuffer) {
    let sleep_time = time::Duration::from_millis(250);

    loop {
        thread::sleep(sleep_time);
    }
}

enum Command {
    Shutdown,
}

fn main() {
    let frame_buffer: FrameBuffer = Arc::new(Mutex::new(im::ImageBuffer::new(600, 600)));
    let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();

    let tx_ref = tx.clone();
    let buffer_ref = frame_buffer.clone();
    thread::spawn(move || {
        render(tx_ref, buffer_ref);
    });

    let buffer_ref = frame_buffer.clone();
    thread::spawn(move || {
        paint(buffer_ref);
    });

    // Add a command queue
    while let Ok(cmd) = rx.recv() {
        match cmd {
            Command::Shutdown => {
                println!("Shutting down");
                break;
            }
        }
    }
}
