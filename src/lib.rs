use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciever) = mpsc::channel();

        let reciever =Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i,Arc::clone(&reciever)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    fn new(id:usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker{
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            job();
        });
        Worker {id, thread}
    }
}