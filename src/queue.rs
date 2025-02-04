use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    // send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // create the channels
        let (send_tasks, recv_tasks) = spmc::channel();
        let (send_output, recv_output) = mpsc::channel();

        // start the worker threads; record their JoinHandles
        let mut workers = Vec::with_capacity(n_workers);
        for _ in 0..n_workers {
            let recv_tasks = recv_tasks.clone();
            let send_output = send_output.clone();

            let worker = thread::spawn(move || {
                WorkQueue::run(recv_tasks, send_output);
            });

            workers.push(worker);
        }

        // return new WorkQueue instance
        WorkQueue {
            send_tasks: Some(send_tasks),
            recv_tasks,
            recv_output,
            workers,
        }
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // The main logic for a worker thread
        loop {
            let task_result = recv_tasks.recv();
            match task_result {
                Ok(task_result) => {
                    if let Some(result) = task_result.run() {
                        send_output.send(result).unwrap();
                    }
                }
                Err(_) => break,
                // task_result will be Err() if the spmc::Sender has been destroyed and no more messages can be received here
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        // send this task to a worker
        self.send_tasks.as_mut().unwrap().send(t)
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // Destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        self.send_tasks = None;

        // Discard messages waiting
        loop {
            match self.recv_tasks.try_recv() {
                Ok(_) => {}
                Err(_) => break,
            }
        }

        // Ensure that the worker threads have exited
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
            // Retrieved from: https://doc.rust-lang.org/std/thread/struct.JoinHandle.html
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
