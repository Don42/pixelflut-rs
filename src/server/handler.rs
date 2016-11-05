use std;
use std::net::TcpStream;
use std::io::{Read, ErrorKind};

use pixelflut::{FrameBuffer, Pixel, put_pixel};


pub trait TcpHandler {
    fn net_loop(&self, mut stream: TcpStream, frame_buffer: FrameBuffer);
}

pub struct BinaryRGBHandler {}

impl TcpHandler for BinaryRGBHandler {
    fn net_loop(&self, mut stream: TcpStream, frame_buffer: FrameBuffer) {
        loop {
            let mut net_buffer: [u8; 7] = [0; 7];
            let res = stream.read_exact(&mut net_buffer);
            match res {
                Ok(_) => {
                    let pixel = Pixel::from_rgb_slice(&net_buffer);
                    put_pixel(pixel, &frame_buffer)
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
}

pub struct BinaryRGBAHandler {}

impl TcpHandler for BinaryRGBAHandler {
    fn net_loop(&self, mut stream: TcpStream, frame_buffer: FrameBuffer) {
        loop {
            let mut net_buffer: [u8; 8] = [0; 8];
            let res = stream.read_exact(&mut net_buffer);
            match res {
                Ok(_) => {
                    let pixel = Pixel::from_rgba_slice(&net_buffer);
                    put_pixel(pixel, &frame_buffer)
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
}



#[derive(Debug)]
pub enum ConnectionType {
    BinaryRGB,
    BinaryRGBA,
    ASCII,
}

pub fn get_connection_type(stream: &mut TcpStream) -> std::io::Result<self::ConnectionType> {
    use self::ConnectionType::*;
    use std::io::{Error, ErrorKind};
    use std::io::Read;
    let mut type_buffer: [u8; 1] = [0; 1];
    let res = stream.read_exact(&mut type_buffer);
    match res {
        Ok(_) => {
            match type_buffer[0] {
                0x00 => Ok(BinaryRGB),
                0x01 => Ok(BinaryRGBA),
                0x02 => Ok(ASCII),
                _ => Err(Error::new(ErrorKind::Other, "Markerbyte not recognized")),
            }
        }
        Err(e) => Err(e),
    }
}
