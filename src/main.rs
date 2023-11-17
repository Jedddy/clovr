use std::{
    net::{TcpListener, TcpStream},
    io::Write,
    fs
};
use clovr::http::read_data;


fn main() {
    let mut round_robin = 0;
    let servers_string = fs::read_to_string("servers.txt")
        .expect("Could not read servers.txt");

    let servers = servers_string
        .split("\n")
        .map(|l| l.trim())
        .collect::<Vec<_>>();

    let server_count = servers.len();

    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Could not bind to address");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let data = read_data(&stream);

        let mut server = TcpStream::connect(servers[round_robin]).unwrap();
        server.write_all(&data).unwrap();

        let mut data = read_data(&server);
        let mut body = read_data(&server);
        data.append(&mut body);

        stream.write_all(&data).unwrap();

        round_robin += 1;

        if round_robin > server_count - 1 {
            round_robin = 0;
        }
    }

}
