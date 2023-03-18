use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::{anyhow, Result};

use super::client_config::ClientConfig;

pub struct Server {
    addr: String,
    port: u16,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new(addr: String, port: u16) -> Self {
        Server {
            addr,
            port,
            listener: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        self.listener = Some(listener);
        Ok(())
    }

    pub fn recv(&self) -> Result<String> {
        let port = *&self.port;
        if let Some(listener) = &self.listener {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    if let Some(url) = Server::handle_connection(stream) {
                        let redirect_url = ClientConfig::get_local_server_addr(port);
                        return Ok(format!("{}{}", &redirect_url, url));
                    }
                };
            }
        }
        Err(anyhow!("Failed redirection"))
    }

    fn handle_connection(mut stream: TcpStream) -> Option<String> {
        // The request will be quite large (> 512) so just assign plenty just in case
        let mut buffer = [0; 1000];
        let _ = stream.read(&mut buffer).unwrap();

        // convert buffer into string and 'parse' the URL
        match String::from_utf8(buffer.to_vec()) {
            Ok(request) => {
                println!("======================== : {}", request);
                let split: Vec<&str> = request.split_whitespace().collect();

                if split.len() > 1 {
                    Server::respond_with_success(stream);
                    return Some(split[1].to_string());
                }

                Server::respond_with_error("Malformed request".to_string(), stream);
            }
            Err(e) => {
                Server::respond_with_error(format!("Invalid UTF-8 sequence: {}", e), stream);
            }
        };

        None
    }

    fn respond_with_success(mut stream: TcpStream) {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn respond_with_error(error_message: String, mut stream: TcpStream) {
        println!("Error: {}", error_message);
        let response = format!(
            "HTTP/1.1 400 Bad Request\r\n\r\n400 - Bad Request - {}",
            error_message
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
