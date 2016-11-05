use std;
use std::net::{TcpListener, TcpStream};

use pixelflut::FrameBuffer;

mod handler;


fn handle_connection(mut stream: TcpStream, frame_buffer: FrameBuffer) {
    use self::handler::{ConnectionType, TcpHandler};
    let connection_type = handler::get_connection_type(&mut stream)
        .unwrap_or(ConnectionType::BinaryRGB);
    match connection_type {
        ConnectionType::BinaryRGB => {
            let con_handler = handler::BinaryRGBHandler {};
            con_handler.net_loop(stream, frame_buffer);
        }

        ConnectionType::BinaryRGBA => {
            let con_handler = handler::BinaryRGBAHandler {};
            con_handler.net_loop(stream, frame_buffer);
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
