use std::{
    fs::{self, File},
    io::Write,
    net::{TcpListener, TcpStream},
};

pub trait Handler {
    fn handle(&self, stream: TcpStream);
}

#[derive(Clone)]
pub struct TcpServer {
    host: String,
    port: i32,

    file_path: Option<String>,
}

impl TcpServer {
    pub fn new(host: String, port: i32) -> Self {
        Self {
            host,
            port,
            file_path: None,
        }
    }

    pub fn file_path(self) -> Option<String> {
        self.file_path
    }

    pub fn start(&self) {
        let listener = match TcpListener::bind(format!("{}:{}", self.host, self.port)) {
            Ok(l) => l,
            Err(e) => {
                println!("Error with starting server {e:?}");
                panic!("cannot bind port");
            }
        };

        println!("server started on {}:{} port", self.host, self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(s) => self.clone().handle(s),
                Err(e) => {
                    println!("unexpected error occured on incoming stream {e:?}");
                }
            }
        }
    }

    pub fn upload_file(&mut self, path: String) {
        let file = File::open(path.clone());

        match file {
            Ok(_) => {
                println!("file opened path={path}");
                self.file_path = Some(path);
            }
            Err(e) => {
                println!("cannot open file path={path} ({e})")
            }
        }
    }

    pub fn clear_file(&mut self) {
        self.file_path = None;
    }

    fn handle(&self, mut stream: TcpStream) {
        let _ = match self.file_path {
            None => Err("file didnt uploaded"),
            Some(_) => Ok(()),
        };

        let bytes = fs::read(self.file_path.clone().unwrap());

        let _ = match bytes {
            Ok(bytes) => stream.write(&bytes),
            Err(_) => stream.write(b"file doesnt exists"),
        };
    }
}
