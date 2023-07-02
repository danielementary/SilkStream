use std::net::UdpSocket;

use silk_stream::Datagram;

fn main() {
    println!("Hello, client!");

    let socket = UdpSocket::bind("127.0.0.1:8081").expect("Failed to bind socket");

    println!(
        "About to send a message on {}",
        socket.local_addr().unwrap()
    );

    let datagram = Datagram::new("Hey there!".as_bytes());

    socket
        .send_to(datagram.as_bytes(), "127.0.0.1:8080")
        .expect("Failed to send message");
}
