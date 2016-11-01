extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image as im;
extern crate byteorder;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::{time, thread};
use std::net::{TcpListener, TcpStream};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};


type FrameBuffer = Arc<Mutex<im::RgbaImage>>;

#[derive(Debug)]
struct Pixel {
    x: u16,
    y: u16,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    fn from_slice(buffer: &[u8]) -> Pixel {
        use std::io::Cursor;
        use byteorder::{NetworkEndian, ReadBytesExt};
        let mut rdr = Cursor::new(buffer);
        Pixel {
            x: rdr.read_u16::<NetworkEndian>().unwrap(),
            y: rdr.read_u16::<NetworkEndian>().unwrap(),
            r: rdr.read_u8().unwrap(),
            g: rdr.read_u8().unwrap(),
            b: rdr.read_u8().unwrap(),
            a: 255,
        }
    }

    fn is_on_canvas(&self, width: u32, height: u32) -> bool {
        (self.x as u32) < width && (self.y as u32) < height
    }
}


struct App {
    gl: GlGraphics,
    frame_buffer: FrameBuffer,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let frame_buffer = self.frame_buffer.lock().unwrap();

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let mut texture = opengl_graphics::Texture::from_image(&frame_buffer,
                                                               &TextureSettings::new());
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BLACK, gl);
            texture.update(&frame_buffer);
            graphics::image(&texture, c.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

fn renderer(tx: Sender<Command>, frame_buffer: FrameBuffer) {

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


fn handle_client(mut stream: TcpStream, frame_buffer: FrameBuffer) {
    use std::io::Read;
    use std::io::ErrorKind;
    loop {
        let mut net_buffer: [u8; 7] = [0; 7];
        let res = stream.read_exact(&mut net_buffer);
        match res {
            Ok(_) => {
                let pixel = Pixel::from_slice(&net_buffer);
                let mut fb = frame_buffer.lock().unwrap();
                let (width, height) = fb.dimensions();

                if pixel.is_on_canvas(width, height) {
                    fb.put_pixel((pixel.x as u32),
                                 (pixel.y as u32),
                                 im::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
                };

            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                println!("Connection blocked: {:?}", e);
                break;
            }
            Err(e) => {
                println!("Connection error: {:?}", e);
                println!("Error Kind {:?}", e.kind());
                break;
            }
        }
    }
}

fn listener(frame_buffer: FrameBuffer) {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
    let timeout = Some(std::time::Duration::new(5, 0));
    for stream in listener.incoming() {
        let buffer_ref = frame_buffer.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    stream.set_read_timeout(timeout).unwrap();
                    handle_client(stream, buffer_ref)
                });
            }
            Err(_) => panic!("Connection failed"),
        }
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
        renderer(tx_ref, buffer_ref);
    });


    let buffer_ref = frame_buffer.clone();
    thread::spawn(move || {
        listener(buffer_ref);
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
