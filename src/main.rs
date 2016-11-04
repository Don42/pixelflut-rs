use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

mod pixelflut;
use pixelflut::{FrameBuffer, build_frame_buffer, Command};

mod canvas;
use canvas::renderer;

mod server;
use server::listener;


fn main() {
    let frame_buffer: FrameBuffer = build_frame_buffer(1366, 768);
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
