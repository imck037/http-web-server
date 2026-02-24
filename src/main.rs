use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2020").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_client(stream);
    }
}

#[allow(unused)]
fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{http_request:#?}");
    let contents = fs::read_to_string("index.html").unwrap();
    let lenth = contents.len();
    let status_line = "http/1.1 200 ok";
    let response = format!("{status_line}\r\nContent-Lentg: {lenth}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
