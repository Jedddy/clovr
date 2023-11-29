use std::{
    net::{TcpStream, Shutdown},
    io::Write,
};
use crate::threadpool::ThreadPool;
use crate::http::read_data;

pub struct LoadBalancer {
    servers: Vec<String>,
    server_count: usize,
    pool: ThreadPool
}

impl LoadBalancer {
    pub fn new(servers: Vec<String>, pool: ThreadPool) -> Self {
        let server_count = servers.len();
        Self { servers, pool, server_count }
    }

    pub fn handle(&mut self, mut stream: TcpStream) {
        if self.server_count > 1 {
            self.servers.rotate_left(1);
        }

        let server = self.servers[0].clone();

        self.pool.execute(move || {
            let data = read_data(&stream);

            if data.len() < 1 {
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }

            let mut server = match TcpStream::connect(server) {
                Ok(server) => server,
                Err(_) => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    return
                }
            };

            server.write_all(&data).unwrap();

            let mut data = read_data(&server);
            let mut body = read_data(&server);
            data.append(&mut body);

            stream.write_all(&data).unwrap();
            stream.flush().unwrap();
        })
    }
}
