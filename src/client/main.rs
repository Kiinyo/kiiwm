use std::{env, os::unix::net::UnixDatagram};

fn send_to_daemon (client_name: String, packet: Vec<u8>, expecting_answer: bool) -> Vec<u8> {
    // Create our socket address
    let address = format! ("/tmp/kiiwm.{}", client_name);
    // Preempting our return value
    let mut buffer: Vec<u8> = vec![0; 32];
    // Create the socket to be sent from
    let sock = match UnixDatagram::bind(&address) {
        Ok(sock) => sock,
        Err(_) => {
            std::fs::remove_file(&address).unwrap();
            match UnixDatagram::bind(&address) {
                Ok(socket) => socket,
                Err(e) => {
                    println! ("We couldn't bind to the socket '{}'! If you keep getting this error, message me on Discord at Kiinyo#7783!", address);
                    panic! ("Error: {:?}", e);
                }
            }
        }
    };
    // Connect to the daemon's socket
    match sock.connect("/tmp/kiiwm.daemon") {
        Ok(_) => {},
        Err(e) => {
            // Cleaning up the socket for later use
            std::fs::remove_file(&address).unwrap();
            // Letting the user know what happened.
            println! ("We couldn't connect to daemon's socket! If you keep getting this error, message me on Discord at Kiinyo#7783!");
            panic! ("Error: {:?}", e);
        }
    }
    // Send via the socket
    match sock.send(&packet) {
        Ok(_) => {},
        Err(e) => {
            std::fs::remove_file(&address).unwrap();
            println! ("We couldn't send the packet to the daemon! If you keep getting this error, message me on Discord at Kiinyo#7783!");
            panic! ("Error: {:?}", e);
        }        
    }
    // Now wait if we're expecting an answer
    if expecting_answer {
        match sock.recv_from(&mut buffer) {
            Ok(_) => {
                return buffer;
            },
            Err(e) => {
                std::fs::remove_file(&address).unwrap();
                println! ("Sorry, looks like we couldn't return an answer from the daemon! If you keep getting this error, message me on Discord at Kiinyo#7783!");
                panic! ("Error: {:?}", e);                
            }
        }
    }
    // Clean up the socket
    std::fs::remove_file(&address).unwrap();
    // Return the buffer
    return buffer;
}

fn main() {
    // Collect args passed to client
    let args: Vec<String> = env::args().collect();
    // Save the length for later
    let length = args.len();
    // No argument was provided
    if args.len() == 1 {
        println! ("Welcome to Kiinyo's WM! For a list of commands type \"kii -h, --help\"");
    } else {
        match args[1].as_str() {
            // We've entered the help module
            "-h" | "--help" => {
                println!("Usage: kii [--MODULES, -module] <a r g s>");
                println!("");
                println!("MODULES: Add 'help' after the module to get more info!");
                println!("");
                println!("  -h, --help          Displays help module. (You're here already!)");
                println!("  -a, --audio         Enters the audio module, used for controlling volume!");
            }
            "-a" | "--audio" => {
                if length > 2 {
                    match args[2].as_str() {
                        "set" => {
                            if length > 3 {
                                match args[3].parse::<u8>() {
                                    Ok(number) => {
                                        let volume = send_to_daemon("audio".to_string(), vec![1, 1, number], true);
                                        match volume[0] {
                                            0..=100 => println!("Volume set to {}%", volume[0]),
                                            255 => println!("Error: There was an issue setting the volume!"),
                                            _ => println!("Error: You shouldn't be seeing this, nice job!")
                                        }
                                    }
                                    Err(_) => {
                                        println!("'{}' is not a valid number! Try '30'", args[3]);
                                    }
                                };
                            } else {
                                println! ("Error: Please supply a number to be 'set'!")
                            }
                        },
                        "get" => {
                            let volume = send_to_daemon("audio".to_string(), vec![1, 2], true);
                            match volume[0] {
                                0..=100 => println!("Volume is currently {}%", volume[0]),
                                255 => println!("Error: There was an issue getting the volume!"),
                                _ => println!("Error: You shouldn't be seeing this, nice job!")
                            }
                        },
                        "inc" => {
                            if length > 3 {
                                match args[3].parse::<u8>() {
                                    Ok(number) => {
                                        let volume = send_to_daemon("audio".to_string(), vec![1, 3, number], true);
                                        match volume[0] {
                                            0..=100 => println!("Volume set from {}% to {}%", volume[0], volume[1]),
                                            255 => println!("Error: There was an issue setting the volume!"),
                                            _ => println!("Error: You shouldn't be seeing this, nice job!")
                                        }
                                    }
                                    Err(_) => {
                                        println!("'{}' is not a valid number! Try '20'", args[3]);
                                    }
                                };
                            } else {
                                println! ("Error: Please supply a number to 'inc'rease volume by!")
                            }
                        },
                        "dec" => {
                            if length > 3 {
                                match args[3].parse::<u8>() {
                                    Ok(number) => {
                                        let volume = send_to_daemon("audio".to_string(), vec![1, 4, number], true);
                                        match volume[0] {
                                            0..=100 => println!("Volume set from {}% to {}%", volume[0], volume[1]),
                                            255 => println!("Error: There was an issue setting the volume!"),
                                            _ => println!("Error: You shouldn't be seeing this, nice job!")
                                        }
                                    }
                                    Err(_) => {
                                        println!("'{}' is not a valid number! Try '10'", args[3]);
                                    }
                                };
                            } else {
                                println! ("Error: Please supply a number to 'inc'rease volume by!")
                            }
                        },

                        "help" | "h" | "-h" | "--help" => {
                            println!("Welcome to the Audio module! Valid arguments include:");
                            println!("");
                            println!("  get     'Get's the current [volume]%!");
                            println!("  set     'Set's the current [volume]%!");
                            println!("  inc #   'Inc'reases the current volume by [#]%!");
                            println!("  dec #   'Dec'reases the current volume by [#]%!");
                        },

                        _ => {
                            println!("Error: {} was an invalid argument for the Audio module!", args[2]);
                            println!("");
                            println!("  Try 'kii --audio help' for more information!");
                        }
                    }
                } else {
                    println!("Error: No arguments given for the Audio module!");
                    println!("");
                    println!("  Try 'kii --audio help' for more information!");
                }
            }
            "-d" | "--diagnostic" => {
                if length > 2 {
                    match args[2].as_str() {
                        "ram" | "RAM" => {
                            let ram = send_to_daemon("diagnostic".to_string(), vec![2, 1], true);
                            println! ("RAM usage {}% of {}GB", ram[0], ram[1]);
                        }
                        _ => {
                            println! ("Aw heck");
                        }
                    }
                }
            }
            _ => println! ("Error: '{}' wasn't a valid module. See 'kii --help' for more information!", args[1])
        }
    }
}