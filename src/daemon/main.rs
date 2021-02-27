use std::os::unix::net::UnixDatagram;

fn main() {
    // Hello world so I don't get confused
    println!("Hello, I am the daemon!");
    // Create the socket! If there's a previous instance of this socket
    // delete it and keep going! There shouldn't be more than one of
    // these running at a time!
    let sockiit = match UnixDatagram::bind("/tmp/kiiwm.daemon") {
        Ok(socket) => socket,
        Err(_) => {
            std::fs::remove_file("/tmp/kiiwm.daemon").unwrap();
            match UnixDatagram::bind("/tmp/kiiwm.daemon") {
                Ok(socket) => socket,
                Err(e) => panic!("{:?}", e)
            }
        }
    };
    // Double check the address of the socket
    let addr = sockiit.local_addr().expect("Couldn't get local address");
    println!("Sockiit Address: {:?}", addr);
    // Wait for something and then write to it.
    loop {
        let mut buf: Vec<u8> = vec![0;32];
        let (_, address) = sockiit.recv_from(&mut buf).unwrap();
        println!("socket {:?} sent {:?}", address, &buf);
        if buf[0] == 1 {break}
    }
    // Before it ends, remove the socket from the filesystem
    // for future use and to let the clients know the daemon
    // is no longer active! .unwrap because there's no way this
    // should not be here.
    std::fs::remove_file("/tmp/kiiwm.daemon").unwrap();
}
