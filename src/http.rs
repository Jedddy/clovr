use std::net::TcpStream;
use std::io::{prelude::*, BufReader};


pub fn read_data(mut stream: &TcpStream) -> Vec<u8> {
    let mut data = Vec::new();
    let buf_size = 256;
    let mut buf = BufReader::new(&mut stream);

    loop {
        let mut b = vec![0; buf_size];

        match buf.read(&mut b) {
            Ok(n) => {
                if n == 0 {
                    break
                }

                if n < buf_size {
                    data.append(&mut b[..n].to_vec());
                    break
                }

                data.append(&mut b);
            },
            Err(e) => panic!("Error reading data: {}", e)
        }
    }

    data
}