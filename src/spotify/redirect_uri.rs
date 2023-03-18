use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use anyhow::{anyhow, Result};
use rspotify::AuthCodeSpotify;

use super::client_config::ClientConfig;

pub fn redirect_uri_web_server(spotify_oauth: &AuthCodeSpotify, port: u16) -> Result<String> {
    let listener = TcpListener::bind(&(format!("127.0.0.1:{}", port)))?;

    request_token(spotify_oauth)?;

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            if let Some(url) = handle_connection(stream) {
                let redirect_url = ClientConfig::get_local_server_addr(port);
                return Ok(format!("{}{}", &redirect_url, url));
            }
        };
    }
    Err(anyhow!("Failed redirection"))
}

fn handle_connection(mut stream: TcpStream) -> Option<String> {
    let mut buffer = [0; 1000];
    let _ = stream.read(&mut buffer).unwrap();

    match String::from_utf8(buffer.to_vec()) {
        Ok(request) => {
            println!("======================== : {}", request);
            let split: Vec<&str> = request.split_whitespace().collect();

            if split.len() > 1 {
                respond_with_success(stream);
                return Some(split[1].to_string());
            }

            respond_with_error("Malformed request".to_string(), stream);
        }
        Err(e) => {
            respond_with_error(format!("Invalid UTF-8 sequence: {}", e), stream);
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

pub fn request_token(spotify_oauth: &AuthCodeSpotify) -> Result<()> {
    let auth_url = spotify_oauth.get_authorize_url(true)?;
    open::that(&auth_url)?;
    Ok(())
}
