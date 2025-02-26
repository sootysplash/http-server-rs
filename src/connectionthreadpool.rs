use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};

use crate::executor::Executor;


pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    
    pub fn new(thread_count : usize) -> ThreadPool {
        assert!(thread_count > 0);
        
        let mut workers = Vec::with_capacity(thread_count);
        let (sender, receiver) = mpsc::channel();

        let arc_rec = Arc::new(Mutex::new(receiver));
        for _ in 0..thread_count {
            workers.push(Worker::new(
                // id + 1,
                Arc::clone(&arc_rec)));
        }
        // println!("created {} worker threads", workers.len());
        let sender = Some(sender);
        return ThreadPool {sender, workers};
    }
    
}

impl Executor for ThreadPool {
    
    fn execute<F>(&self, job : F)
    where F : FnOnce() + Send + 'static {
        self.sender.as_ref().unwrap().send(Box::new(job)).unwrap();
    }
    
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        while self.workers.len() > 0 {
            let worker = self.workers.remove(0);
            worker.thread.join().unwrap();
            // println!("id: {} has been shut down", worker.id);
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    // id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(
        // id: usize, 
        employer : Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move|| {
            loop {
                let value = employer.lock().unwrap().recv();
                match value {
                    Ok(job) => {
                        // println!("id: {} got a job", id);
                        job();
                    }
                    Err(_) => {
                        // println!("id: {} receiver has shutdown", id);
                        break;
                    }
                }
            }
        });
            
        Worker { 
            // id,
            thread }
    }
}