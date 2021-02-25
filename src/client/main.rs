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
    sock.connect("/tmp/sockiit").expect("Couldn't connect");
    let msg1 = vec![0,1,2,3];
    sock.send(&msg1).expect("send_to function failed");
    let msg1 = vec![1,1,2,3];
    sock.send(&msg1).expect("send_to function failed");
}