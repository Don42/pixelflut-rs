extern crate rand;
extern crate piston_window;
extern crate sdl2_window;
extern crate image as im;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self,Sender,Receiver};
use std::{time, thread};

use piston_window::*;
use sdl2_window::Sdl2Window;

type FrameBuffer = Arc<Mutex<im::ImageBuffer<im::Rgba<u8>, Vec<u8>>>>;

fn render(tx: Sender<Command>, frame_buffer: FrameBuffer) {

	let opengl = OpenGL::V3_2;
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("piston", [512; 2])
		.exit_on_esc(true)
		.opengl(opengl)
        .build()
        .unwrap();
	window.set_max_fps(30);
	window.set_ups(30);

	let mut texture = Texture::from_image(&mut window.factory,
		&frame_buffer.lock().unwrap(),
		&TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
		if let Event::Render(_) = e {
			texture.update(&mut window.encoder, &frame_buffer.lock().unwrap()).unwrap();

			window.draw_2d(&e, |c, g| {
				// clear([0.0, 0.0, 0.0, 1.0], g);
				image(&texture, c.transform, g);
			});
		}
    }
    tx.send(Command::Shutdown).unwrap();
}

fn paint(frame_buffer: FrameBuffer) {
    let sleep_time = time::Duration::from_millis(20);
	let mut x: u32 = 0;
    loop {
        thread::sleep(sleep_time);
		frame_buffer.lock().unwrap().put_pixel(x, x+15,
			im::Rgba([0, 255, 255, 255]));
		x = (x + 1) % 1024;
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
