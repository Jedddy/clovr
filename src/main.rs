use std::{
    net::TcpListener,
    fs,
};
use clovr::threadpool::ThreadPool;
use clovr::load_balancer::LoadBalancer;


fn main() {
    let servers_string = fs::read_to_string("servers.txt")
        .expect("Could not read servers.txt");

    let mut servers = servers_string
        .split("\n")
        .map(|l| l.trim().to_owned())
        .collect::<Vec<_>>();

    let num_servers = servers.len();

    let listener = TcpListener::bind("0.0.0.0:80")
        .expect("Could not bind to address");

    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        if num_servers > 1 {
            servers.rotate_left(1);
        }

        let stream = stream.unwrap();

        let server = servers[0].clone();

        pool.execute(move || LoadBalancer::handle(stream, &server))
    }
}
