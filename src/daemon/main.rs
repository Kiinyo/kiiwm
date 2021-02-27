use std::{
    os::unix::net::UnixDatagram, 
    process::Command};

/// Uses 'amixer' to set the Master volume of the device
fn set_volume(volume: u8) {

    let volume_arg = format!("{}%", volume);

    match Command::new("amixer")
    .arg("set")
    .arg("Master")
    .arg(&volume_arg)
    .output() {
        Ok(_) => println!("a{}",volume),
        Err(e) => println!("Error: Something went wrong with setting the volume via amixer! {:?}", e)
    }
}
/// Uses 'amixer' to get the Master volume of the device
fn get_volume () -> u8 {
    let audio = Command::new("amixer")
    .output()
    .unwrap()
    .stdout;
    
    let mut percent = Vec::new();

    let mut tracker: usize = 0;
    for (c, character) in audio.iter().enumerate() {
        match character {
            b'[' => {
                tracker = c + 1;
            },
            b']' => {

                percent.extend_from_slice(&audio[tracker..c-1]);

                break;
            },
            _ => {}
        }
    }
    parse_number_from_utf8(percent) as u8
}
/// Clamps a value between a max and a min
fn clamp (min: isize, value: isize, max: isize) -> isize {
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}
/// Returns a number from a given Vec<u8>
fn parse_number_from_utf8 (vector: Vec<u8>) -> isize {
    let mut place: isize = 1;
    let mut number: isize = 0;

    for digit in vector.iter().rev() {
        match digit {
            48..=57 => {                
                number += (digit - 48) as isize * place;
                place *= 10;
            }
            _ => {
                println! ("Error: Invalid number given in parse_number_from_utf8 - {}", digit);
                number = 0;
                break;
            }
        }
    }

    return number;
}

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
    // Wait for something and then write to it.
    'messages: loop {
        let mut buf: Vec<u8> = vec![0;32];
        let (_, address) = sockiit.recv_from(&mut buf).unwrap();

        // Debugging for sent packets
        //println!("socket {:?} sent {:?}", address, &buf);

        // We might get more than one input somehow so this is here
        // to make sure we get all of them!
        let mut tracker = 0;
        'buffers: loop {
            if tracker >= 32 {break 'buffers;}
            match buf[0 + tracker] {
                // We're in the audio section
                1 => {
                    match buf[1 + tracker] {
                        // We're setting the volume
                        1 => {
                            // Get the volume
                            let volume = buf[2 + tracker];
                            // Run the amixer program here
                            set_volume(volume);
                            tracker += 3;
                        }
                        // We're getting the volume!
                        2 => {
                            let volume = get_volume();
                            match sockiit.send_to(&vec![volume], address.as_pathname().unwrap()) {
                                Ok(_) => {
                                    println! ("Success: Audio volume {}% was sent!", volume)
                                }
                                Err(e) => {
                                    println! ("Error: Couldn't send back to client! {}", e)
                                }
                            }
                            tracker += 2;
                        }
                        // We're incrementing the volume!
                        3 => {
                            // Get the current volume.
                            let p_volume = get_volume();
                            // Applying the proper inc value
                            let volume = match buf[2 + tracker] {
                                // Addition
                                1 => clamp(0, p_volume as isize + buf[3 + tracker] as isize, 100) as u8,
                                2 => clamp(0, p_volume as isize - buf[3 + tracker] as isize, 100) as u8,
                                _ => {
                                    println!("Error: Invalid inc argument for volume!");
                                    p_volume
                                }
                            };
                            // Set the volume
                            set_volume(volume);
                            // Tell the client we set the volume
                            match sockiit.send_to(&vec![volume], address.as_pathname().unwrap()) {
                                Ok(_) => {
                                    println! ("Success: Audio volume changed from {}% to {}%!", p_volume, volume)
                                }
                                Err(e) => {
                                    println! ("Error: Couldn't send volume back to client! {}", e)
                                }
                            }
                            // Update thet racker
                            tracker += 4;
                        }
                        // Junk audio code!
                        _ => {
                            println! ("Error: Invalid audio code! Got: {}", buf[1 + tracker]);
                            break 'buffers;
                        }
                    }
                }
                // Quit command
                255 => {
                    break 'messages;
                }
                _ => {
                    break 'buffers;
                }
            }
        }
    }
    // Before it ends, remove the socket from the filesystem
    // for future use and to let the clients know the daemon
    // is no longer active! .unwrap because there's no way this
    // should not be here.
    std::fs::remove_file("/tmp/kiiwm.daemon").unwrap();
}
