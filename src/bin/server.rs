use std::net::UdpSocket;

fn main() {
    println!("Hello, server!");

    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind socket");

    println!("Listening on {}", socket.local_addr().unwrap());

    let mut buf = [0u8; 1024];

    loop {
        let (bytes_read, client_addr) = socket
            .recv_from(&mut buf)
            .expect("Failed to read from socket");

        let received_message = &buf[..bytes_read];
        println!(
            "Received message from {}: {:?}",
            client_addr, received_message
        );
    }
}
