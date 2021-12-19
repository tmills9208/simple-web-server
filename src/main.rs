/*
Project: simple tcp web server
Author: Tyler Mills, tmills9208
Description:
    A simple web server that has an index and 404 page. It has multi-threaded tasks to send HTML to clients, and shuts down after 5 calls, utilizing the drop functionality of the threads.
Version History:
    0.1 - init
*/

// Part 1
use std::net::{TcpListener, TcpStream};
use std::io::prelude::{Read, Write};
use std::fs;

// Part 2
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:8787").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();
        
        println!("Connection established.");
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // println!(
    //     "Request: {}\r\n",
    //     String::from_utf8_lossy(&buffer[..])
    // );

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        }
        else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = 
        fs::read_to_string(filename).unwrap();

    /* Offending code:
        let response = format!(
            "{}\r\nContent-Length: {}\r\n{}",
            status_line,
            contents.len(), <--
            contents
        );
    */
    // working!
    let response = format!(
        "{}\r\n\r\n{}",
        status_line,
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
