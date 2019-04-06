use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}


// 1. The ThreadPool will create a channel and hold on to the sending side of the channel.
// 2. Each Worker will hold on to the receiving side of the channel.
// 3. We’ll create a new Job struct that will hold the closures we want to send down the channel.
// 4. The execute method will send the job it wants to execute down the sending side of the channel.
// 5. In its thread, the Worker will loop over its receiving side of the channel
//   and execute the closures of any jobs it receives.
struct Job;

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    // The Arc type will let multiple workers own the receiver,
    // and Mutex will ensure that only one worker gets a job from the receiver at a time.
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      // For each new worker, we clone the Arc to bump the reference count 
      // so the workers can share ownership of the receiving end.
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {

  }
}

// 1. Define a Worker struct that holds an id and a JoinHandle<()>.
// 2. Change ThreadPool to hold a vector of Worker instances.
// 3. Define a Worker::new function that takes an id number and 
//   returns a Worker instance that holds the id and a thread spawned with an empty closure.
// 4. In ThreadPool::new, use the for loop counter to generate an id, 
//   create a new Worker with that id, 
//   and store the worker in the vector.

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    });

    Worker {
      id,
      thread,
    }
  }
}
