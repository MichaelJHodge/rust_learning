//prelude gives us access to traits that let us read from and write
//to the stream.

use server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //Bind returns a new TCPListener instance. We call it bind
    //because in networking we say 'binding to a port'.

    //This function returns a Result<T,E> type because it could fail.

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //We use this to create a new thread pool with a configurable thread count.
    let pool = ThreadPool::new(4);

    //The 'incoming' method returns an iterator that gives us a sequence
    //of streams. One stream represents an open connection between the
    //client and server.
    for stream in listener.incoming() {
        //Calling unwrap just terminates the program without any
        //real error handling if the stream has any errors.
        let stream = stream.unwrap();

        //takes a closure the pool should run for each stream. We need this
        //so it takes the closure and gives it to a thread in the pool to run.
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

//stream parameter is mutable because it might read more
//data than we asked for and save that data for next time we
//ask for it. B/c its internal state could change, it's mutable.

fn handle_connection(mut stream: TcpStream) {
    //the buffer holds the data that is read in. Buffer management
    //would need to be more complicated if we wanted to handle arbitrary requests.
    let mut buffer = [0; 1024];

    //Reads bytes from the TcpStream and puts them in the buffer.
    stream.read(&mut buffer).unwrap();

    //Because we’re reading raw bytes into the buffer, we transform get into a byte string
    //by adding the b"" byte string syntax at the start of the content data.
    let get = b"GET / HTTP/1.1\r\n";

    //Then we check if buffer starts with the bytes in get. If so, it means
    //we have received a well-formed request. the let blocks only return the appropriate
    //values for the status line and filename in a tuple; we then use destructuring to assign these
    //two values to status_line and filename using a pattern in the let statement, as discussed in Chapter 18.

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    //reads the file contents

    let contents = fs::read_to_string(filename).unwrap();

    //uses format! to add the file's contents as the body of the success response. To ensure
    //a valid HTTP response, we add the content-length header which is set to the size of
    //our response body (in this case the size of hello.html)
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    //the write method on stream takes a &[u8] and sends those bytes down the connection.
    stream.write(response.as_bytes()).unwrap();

    //flush will wait and prevent the program from continuing until all the bytes are written to the connection.

    stream.flush().unwrap();
}
