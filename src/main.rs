use std::{
    net::TcpListener,
    fs,
};
use clovr::threadpool::ThreadPool;
use clovr::load_balancer::LoadBalancer;


fn main() {
    let servers_string = fs::read_to_string("servers.txt")
        .expect("Could not read servers.txt");

    let servers = servers_string
        .split("\n")
        .map(|l| l.trim().to_owned())
        .collect::<Vec<_>>();

    let pool = ThreadPool::new(10);
    let mut load_balancer = LoadBalancer::new(servers, pool);

    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)
        .expect("Could not bind to address");

    println!("Listening in: {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => load_balancer.handle(stream),
            Err(e) => eprintln!("An error occured: {e}")
        }
    }
}
