use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    process, fmt::Display,
    sync::{mpsc, Mutex, Arc},
    time::Duration,
};

#[allow(dead_code)]
pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap_or_else(|e| {
        eprintln!("Couldn't bind the TCP Listener: {e}");
        std::process::exit(-1);
    });

    let pool = ThreadPool::build(4).unwrap_or_else(|e| {
        eprintln!("Cannot create ThreadPool: {e}");
        process::exit(-1);
    });

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        
        pool.execute(|| handle_stream(stream));
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
    "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = read_file_contents(filename.to_string());
    let contents_length = contents.len();
    let response = 
    format!("{status_line}\r\nContent-Length: {contents_length} \r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap_or_else(|e| {
    eprintln!("Couldn't write into stream: {e}");
});
}

fn read_file_contents(name: String) -> String {
    let file_path = format!("resources/{name}");
    
    let html = fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Couldn't read file '{name}' from resources folder: {e}");
        return "".to_string();
    });

    html
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size <= 0 {
            return Err(ThreadPoolError { info: "ThreadPool size must be greater than 0.".to_string() });
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id as u32, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender: Some(sender) })
    }

    pub fn execute<F> (&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap_or_else(|e| {
            eprintln!("Couldn't send job to thread through channel: {e}");
        });
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: u32,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let t = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job: executing...");

                    job();                
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shutting down.");
                    break;
                }
            }
        });
        
        Worker { id, thread: Some(t) }
    }
}

pub struct ThreadPoolError {
    info: String,
}

impl Display for ThreadPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.info).unwrap_or_else(|e| {
            eprintln!("Error while writing into the ThreadPoolError formatter: {e}");
        });

        Ok(())
    }
}