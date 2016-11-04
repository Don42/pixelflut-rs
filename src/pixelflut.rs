extern crate image as im;
extern crate byteorder;

use std::sync::{Arc, Mutex};

use self::im::ImageBuffer;


pub type FrameBuffer = Arc<Mutex<self::im::RgbaImage>>;

pub fn build_frame_buffer(width: u32, height: u32) -> FrameBuffer {
    Arc::new(Mutex::new(ImageBuffer::new(width, height)))
}

#[derive(Debug)]
pub struct Pixel {
    x: u16,
    y: u16,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub fn from_rgb_slice(buffer: &[u8]) -> Pixel {
        use std::io::Cursor;
        use self::byteorder::{NetworkEndian, ReadBytesExt};
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

    pub fn from_rgba_slice(buffer: &[u8]) -> Pixel {
        use std::io::Cursor;
        use self::byteorder::{NetworkEndian, ReadBytesExt};
        let mut rdr = Cursor::new(buffer);
        Pixel {
            x: rdr.read_u16::<NetworkEndian>().unwrap(),
            y: rdr.read_u16::<NetworkEndian>().unwrap(),
            r: rdr.read_u8().unwrap(),
            g: rdr.read_u8().unwrap(),
            b: rdr.read_u8().unwrap(),
            a: rdr.read_u8().unwrap(),
        }
    }

    pub fn is_on_canvas(&self, width: u32, height: u32) -> bool {
        (self.x as u32) < width && (self.y as u32) < height
    }
}


pub fn put_pixel(pixel: Pixel, frame_buffer: &FrameBuffer) {
    let mut fb = frame_buffer.lock().unwrap();
    let (width, height) = fb.dimensions();

    if pixel.is_on_canvas(width, height) {
        fb.put_pixel((pixel.x as u32),
                     (pixel.y as u32),
                     self::im::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
    };
}

pub enum Command {
    Shutdown,
}
