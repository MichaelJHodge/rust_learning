use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

//Will either be a NewJob variant that holds the Job the thread should run.
//Or it will be a Terminate variant that will cause the thread to exit its loop and stop.

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    ///Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// the 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        //Here we use a smart pointer in order to share ownership across
        //multiple threads and allow them to mutate the value - so we use Arc<Mutex<T>>. the Arc
        //type lets multiple works own the receiver, and the Mutex ensures that only one worker
        //gets a job from the receiver at a time.

        let receiver = Arc::new(Mutex::new(receiver));

        //We put the receiving end of the channel in an Arc and a Mutex.
        //For each new worker, we clone the Arc to bump the reference count so the workers can
        //share ownership of the receiving end.
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        //After creating a new Job instance using the closure we get in this execute function,
        //we send that job down the sending end of the channel and call unwrap on send
        //just incase it fails.
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

//We implement the Drop tait to call join on each of the threads
//in the pool so they can finish the requests before closing.

impl Drop for ThreadPool {
    //We iterate over the workers twice: once to send one Terminate message
    //for each worker and once to call 'join' on each worker's thread.
    //To better understand why we need two separate loops, imagine a scenario with two workers.
    //If we used a single loop to iterate through each worker, on the first iteration a terminate
    //message would be sent down the channel and join called on the first worker’s thread. If that
    //first worker was busy processing a request at that moment, the second worker would pick up the
    //terminate message from the channel and shut down. We would be left waiting on the first worker to
    //shut down, but it never would because the second thread picked up the terminate message. Deadlock!

    fn drop(&mut self) {
        println!("Sending terminate message to all workers!");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            //Ch 17 - the take method on Option takes the Some variant out
            //and leaves None in its place. We use if let to destructure the Some and get the thread.
            //Then we call join on thread. If a worker's thread is already None, we know that the worker has
            //already had its thread cleaned up, so nothing happens in that case.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

//Workers, like employees in a restaurant, wait for code that we'll send later. We
//want the worker structs to fetch code to run from a queue help in the THreadPool and then
//send that code to its thread to run.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        //We call lock on the receiver to acquire the mutex and call unwrap next
        //because getting the lock could fail if the mutex is in a poisoned state -
        //when a thread panics without releasing the lock.

        //We then call recv to receive a Job from the channel.
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
