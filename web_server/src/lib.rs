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

// “Creating Type Synonyms with Type Aliases” section of Chapter 19
type Job = Box<FnOnce() + Send + 'static>;

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
    let job = Box::new(f);
    self.sender.send(job).unwrap();
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
    let thread = thread::spawn(move || {
      loop {
        // Acquiring a lock might fail if the mutex is in a _poisoned_ state,
        // which can happen if
        // some other thread panicked while holding the lock
        // rather than releasing the lock.
        //
        // In this situation,
        // calling unwrap to have this thread panic is the correct action to take.

        // A final unwrap moves past any errors here as well,
        // which might occur if the thread holding the sending side of the channel has shut down,
        // similar to how the send method returns Err if the receiving side shuts down.

        // The call to `recv` blocks, so if there is no job yet,
        // the current thread will wait until a job becomes available.
        // The `Mutex<T>` ensures that only one `Worker` thread at a time is trying to request a job.
        let job = receiver.lock().unwrap().recv().unwrap();
        println!("Worker {} got a job; executing.", id);

        // To call a `FnOnce` closure that is stored in a `Box<T>` (which is what our `Job` type alias is),
        // the closure needs to move itself _out_ of the Box<T> because the closure takes ownership of `self` when we call it.
        //
        // In general, Rust doesn’t allow us to move a value out of a `Box<T>`
        // because Rust doesn’t know how big the value inside the `Box<T>` will be:
        // recall in Chapter 15 that we used `Box<T>` precisely
        // because we had something of an unknown size
        // that we wanted to store in a `Box<T>` to get a value of a known size.
        //
        // As you saw in Listing 17-15, we can write methods that use the syntax `self: Box<Self>`,
        // which allows the method to take ownership of a `Self` value stored in a `Box<T>`.
        // That’s exactly what we want to do here, but unfortunately Rust won’t let us:
        // the part of Rust that implements behavior when a closure is called
        // isn’t implemented using `self: Box<Self>`. 
        // So Rust doesn’t yet understand that it could use `self: Box<Self>` in this situation
        // to take ownership of the closure and move the closure out of the Box<T>.
        (*job)();
      }
    });

    Worker {
      id,
      thread,
    }
  }
}
