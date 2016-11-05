use std;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, ErrorKind};

use pixelflut::{FrameBuffer, Pixel, put_pixel};


trait TcpHandler {
    fn net_loop(&self, mut stream: TcpStream, frame_buffer: FrameBuffer);
}

struct BinaryRGBHandler {}

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

struct BinaryRGBAHandler {}

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
enum ConnectionType {
    BinaryRGB,
    BinaryRGBA,
    ASCII,
}

fn get_connection_type(stream: &mut TcpStream) -> std::io::Result<self::ConnectionType> {
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

fn handle_connection(mut stream: TcpStream, frame_buffer: FrameBuffer) {
    let connection_type = get_connection_type(&mut stream)
        .unwrap_or(self::ConnectionType::BinaryRGB);
    match connection_type {
        self::ConnectionType::BinaryRGB => {
            let handler = BinaryRGBHandler { };
            handler.net_loop(stream, frame_buffer);
        }

        self::ConnectionType::BinaryRGBA => {
            let handler = BinaryRGBAHandler{ };
            handler.net_loop(stream, frame_buffer);
        }

        _ => {}
    };
}



pub fn listener(frame_buffer: FrameBuffer) {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
    let timeout = Some(std::time::Duration::new(5, 0));
    for stream in listener.incoming() {
        let buffer_ref = frame_buffer.clone();
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    stream.set_read_timeout(timeout).unwrap();
                    handle_connection(stream, buffer_ref);
                });
            }
            Err(_) => panic!("Connection failed"),
        }
    }
}