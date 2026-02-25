use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Threadpool {
    /// Create a new Threadpool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panic
    ///
    /// The 'new' function will panic if the size become less than zero
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thrd: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thrd = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job to execute..");

                job();
            }
        });
        Worker { id, thrd }
    }
}
