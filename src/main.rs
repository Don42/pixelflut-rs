extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image as im;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self,Sender,Receiver};
use std::{time, thread};

use rand::Rng;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, TextureSettings };


type FrameBuffer = Arc<Mutex<im::RgbaImage>>;

struct App {
	gl: GlGraphics,
    frame_buffer: FrameBuffer,
}

impl App {
	fn render(&mut self, args: &RenderArgs) {

        let frame_buffer = self.frame_buffer.lock().unwrap();

		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let mut  texture = opengl_graphics::Texture::from_image(
            &frame_buffer,
            &TextureSettings::new());
		self.gl.draw(args.viewport(), |c, gl| {
			graphics::clear(BLACK, gl);
            texture.update(&frame_buffer);
            graphics::image(&texture, c.transform, gl);
		});
    }

	fn update(&mut self, args: &UpdateArgs) {
	}
}

fn render(tx: Sender<Command>, frame_buffer: FrameBuffer) {

	let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("piston", [512; 2])
		.exit_on_esc(true)
		.opengl(opengl)
        .build()
        .unwrap();

	let mut app = App {
        gl: GlGraphics::new(opengl),
        frame_buffer: frame_buffer,
        };


	let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}
    }
    tx.send(Command::Shutdown).unwrap();
}

fn paint(frame_buffer: FrameBuffer) {
    let sleep_time = time::Duration::from_millis(2);

    const CYAN: [u8; 4] = [0, 255, 255, 255];

	let mut i: u32 = 0;
	let mut rng = rand::thread_rng();
    loop {
        thread::sleep(sleep_time);
		let mut fb = frame_buffer.lock().unwrap();
        let (width, height) = fb.dimensions();
        let (x, y) = (rng.gen::<u32>() % width, rng.gen::<u32>() % height);

        fb.put_pixel(x, y,
			im::Rgba([(x % 255) as u8, (y % 255) as u8, rng.gen(), 255]));
		i = (i + 1) % std::cmp::min(width, height);
    }
}

enum Command {
    Shutdown,
}

fn main() {
    let frame_buffer: FrameBuffer = Arc::new(Mutex::new(im::ImageBuffer::new(1366, 768)));
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
