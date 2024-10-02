use clap::{Arg, Command};

use std::io::{self, BufRead, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let matches = Command::new("telnet")
        .arg(
            Arg::new("host")
                .help("The host to connect to")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("port")
                .help("The port to connect to")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("timeout")
                .help("Connection timeout in seconds (e.g., 10, 3)")
                .long("timeout")
                .default_value("10"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<usize>("port").unwrap();

    let timeout_str = matches.get_one::<String>("timeout").unwrap();
    let timeout = match timeout_str.parse::<u64>() {
        Ok(duration) => Duration::from_secs(duration),
        Err(e) => {
            eprintln!("Invalid timeout: {}", e);
            return;
        }
    };

    let addr = format!("{}:{}", host, port);
    println!("Connecting to {}...", addr);

    match connect_with_timeout(&addr, timeout) {
        Ok(mut stream) => {
            println!("Connected to {}", addr);

            // Ctrl+D finale channel
            let (tx, rx) = mpsc::channel();

            let mut stream_clone = stream.try_clone().expect("Failed to clone TCP stream");
            let handle = thread::spawn(move || {
                let mut buffer = [0; 512];
                loop {
                    match stream_clone.read(&mut buffer) {
                        Ok(0) => {
                            println!("Connection closed by server.");
                            let _ = tx.send(()); // Сообщаем главному потоку, что соединение закрыто
                            break;
                        }
                        Ok(n) => {
                            print!("{}", String::from_utf8_lossy(&buffer[..n]));
                            io::stdout().flush().unwrap();
                        }
                        Err(e) => {
                            eprintln!("Error reading from server: {}", e);
                            let _ = tx.send(());
                            break;
                        }
                    }
                }
            });

            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(input) => {
                        if input.is_empty() {
                            break; // checking EOF
                        }

                        if let Err(e) = stream.write_all(input.as_bytes()) {
                            eprintln!("Error writing to server: {}", e);
                            break;
                        }
                        if let Err(e) = stream.write_all(b"\n") {
                            eprintln!("Error writing to server: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        break;
                    }
                }

                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => continue,
                }
            }

            handle.join().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

fn connect_with_timeout<A: ToSocketAddrs>(addr: A, timeout: Duration) -> io::Result<TcpStream> {
    TcpStream::connect_timeout(&addr.to_socket_addrs()?.next().unwrap(), timeout)
}
