use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};
use multithreaded_web_server::ThreadPool;

fn main() {
    // in networking, connecting to a port to listen to is known as "binding to a port"
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // incoming gives an iterator over a sequence of streams.
    // a single stream represents an open connection between the client and the server.
    // we are actually iterating over connection
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
        // connection is closed as part of the drop implementation
    }
}

fn handle_connection(mut stream: TcpStream) {
    // BufReader adds buffering by managing calls to the `std::io::Read` trait methods for us.
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()) // the browser signals the end of an HTTP request by sending two newline characters in a row
        .collect();

    // we are reading the first line because its the one that
    // has the Method Request-URI HTTP-Version CRLF
    // HTTP is text-based, request has the following format
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body
    let first_line = http_request.first().unwrap().as_ref();
    // IMPORTANT: We need to explicitly match on a slice of request_line to pattern match against the string literal values.
    // match doesn't do automatic referencing and dereferencing like the equality method does.
    let (status_line, filename) = match first_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    // println!("Request: {:#?}", http_request)
}

// ===== Improving Throughput with a Thread pool
// https://doc.rust-lang.org/book/ch20-02-multithreaded.html#improving-throughput-with-a-thread-pool