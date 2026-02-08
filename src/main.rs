use std::{
    net::{TcpListener},
    path::PathBuf,
};
use log::{error};
use clap::Parser;

mod media_info;
mod templates;
mod app;
mod assets;

use app::{
    NowPlayingServer
};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the "now playing" JSON metadata log file.
    /// Use the provided jscript_panel_current_playing.js to produce it.
    /// Note that the path may depend in whether or not you're using a portable installation
    /// of foobar2000.
    #[arg(short, long, value_name = "FILE", required = true)]
    json_path: PathBuf,

    /// The local IP address to bind to.
    #[arg(short, long, default_value = "127.0.0.1", value_name="IP")]
    ip_bind: String,

    /// The local port to bind to.
    #[arg(short, long, default_value = "8082", value_name="PORT")]
    port: u16,

    /// The browser overlay display width.
    #[arg(long, default_value_t = 900, value_name="WIDTH")]
    width: u64,

    /// The browser overlay display height.
    #[arg(long, default_value_t = 200, value_name="HEIGHT")]
    height: u64,
}


fn main() {
    env_logger::init();
    let args = Args::parse();

    let ip_full = format!("{}:{}", args.ip_bind, args.port);
    let listener = TcpListener::bind(&ip_full).unwrap_or_else(|err| {
        error!("Error binding a listener to {}. Reason: {}", ip_full, err);
        std::process::exit(1);
    });
    println!("Listening on {}", ip_full);

    let (app_width, app_height) = (args.width, args.height);
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
