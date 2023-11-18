use std::{
    net::TcpStream,
    io::Write,
};
use super::http::read_data;


pub struct LoadBalancer;

impl LoadBalancer {
    pub fn handle(mut stream: TcpStream, server: &str) {
        let data = read_data(&stream);

        let mut server = TcpStream::connect(server).unwrap();
        server.write_all(&data).unwrap();

        let mut data = read_data(&server);
        let mut body = read_data(&server);
        data.append(&mut body);

        stream.write_all(&data).unwrap();
        stream.flush().unwrap();
    }
}