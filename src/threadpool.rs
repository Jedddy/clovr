use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);

        let mut workers = Vec::with_capacity(n);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..n {
            workers.push(Worker::spawn(id, Arc::clone(&receiver)));
        }

        Self { workers, sender: Some(sender) }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Dropping worker {}", worker.id);

            if let Some(worker) = worker.thread.take() {
                worker.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn spawn(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self{
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Failed to lock mutex.")
                .recv();

            match job {
                Ok(job) => job(),
                Err(_) => break
            }
        });

        Self { id, thread: Some(thread) }
    }
}
