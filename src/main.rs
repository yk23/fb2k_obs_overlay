use std::{
    net::{TcpListener},
    path::PathBuf,
};
use log::{error};

mod media_info;
mod templates;
mod app;
mod assets;

use app::{
    NowPlayingServer
};




fn main() {
    env_logger::init();
    let port = 8081;
    let ip_full = format!("{}:{}", "127.0.0.1", port);
    let listener = TcpListener::bind(&ip_full).unwrap_or_else(|err| {
        error!("Error binding a listener to {}. Reason: {}", ip_full, err);
        std::process::exit(1);
    });
    println!("Listening on {}", ip_full);

    let (app_width, app_height) = (900, 200);
    println!("Overlay art (app_width, app_height) = ({app_width}, {app_height})");

    let json_path = PathBuf::from(r"E:\PROJECTS\Streaming\foobar2000\profile\now_playing.json");
    let mut server = NowPlayingServer::new(
        json_path, app_width, app_height
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if let Err(err) = server.serve_connection(stream) {
            error!("Uncaught error while serving connection: {}", err)
        }
    }
}
