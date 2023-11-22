
use std::env;
use std::net::TcpListener;
use std::net::TcpStream;

use std::io::prelude::*;
use std::fs::File;

use std::path::PathBuf;

use std::thread;
use std::time::Duration;


fn build_html_path(file_name : String) -> std::io::Result<PathBuf>
{
    let mut full_path = env::current_dir()?;
    full_path.push(file_name);
    return Ok(full_path);
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let(status_line, file_name) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK", "hello.html")
        
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    println!("Run");
    let my_file = String::from(file_name);
    let my_path = build_html_path(my_file)?;
    let mut file = File::open(my_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, 
        contents.len(),
        contents); 

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}


pub fn run_server()
{
    let listener = TcpListener::bind("127.0.0.1:6848").unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
                continue;
            },
        };

        if let Err(e) = handle_client(stream) {
            eprintln!("Error handling client: {}", e);
        }
    }
}
