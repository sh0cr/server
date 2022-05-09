use std::thread;
use std::time::Duration;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use server::ThreadPool;

fn handle(mut s: TcpStream) {
    let mut buf = [0; 1024];
    s.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else if buf.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "src/hello.html")
    }
     else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-length: {}r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    s.write(response.as_bytes()).unwrap();
    s.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        pool.execute(|| {
            handle(_stream);
        });
    }
}
