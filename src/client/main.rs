use std::os::unix::net::UnixDatagram;
fn main() {
    println!("Hello, I am the client!");

    let sock = match UnixDatagram::unbound() {
        Ok(sock) => sock,
        Err(e) => {
            println!("Couldn't unbound: {:?}", e);
            return
        }
    };
    sock.connect("/tmp/kiiwm.daemon").expect("Couldn't connect");
}