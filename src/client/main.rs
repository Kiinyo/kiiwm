use std::{os::unix::net::UnixDatagram, str::from_utf8, thread::{self, sleep}, time::Duration};
fn main() {
    println!("Hello, I am the client!");

    let sock = match UnixDatagram::bind("/tmp/kiiwm.audio") {
        Ok(sock) => sock,
        Err(e) => {
            std::fs::remove_file("/tmp/kiiwm.audio").unwrap();
            match UnixDatagram::bind("/tmp/kiiwm.audio") {
                Ok(socket) => socket,
                Err(e) => panic!("{:?}", e)
            }
        }
    };
    sock.connect("/tmp/kiiwm.daemon").expect("Couldn't connect");
    
    let mut buf: Vec<u8> = vec![0;3];

    let msg1 = vec![1, 1, 40];
    sock.send(&msg1).expect("send_to function failed");

    std::fs::remove_file("/tmp/kiiwm.audio").unwrap();
}