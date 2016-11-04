extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::sync::mpsc::Sender;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{GlGraphics, OpenGL, TextureSettings};

use pixelflut::{FrameBuffer, Command};


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

    #[allow(dead_code)]
    fn update(&mut self, _: &UpdateArgs) {}
}

pub fn renderer(tx: Sender<Command>, frame_buffer: FrameBuffer) {

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
