use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpStream},
    path::{Path, PathBuf},
};
use askama::Template;
use log::{debug, error};
use crate::assets::load_default_album_art;
use crate::media_info::{
    json::deserialize_json_file,
    metadata::MediaMetadata
};
use crate::templates::*;


enum HttpContentType {
    TextPlain,
    TextHtml,
    ApplicationJson,
    ImagePng,
    ImageJpeg,
    ImageGif,
    ImageWebp,
    TextJavascript,
    TextCSS,
}

impl HttpContentType {
    fn to_string(&self) -> String {
        match self {
            HttpContentType::TextPlain => "text/plain".to_string(),
            HttpContentType::TextHtml => "text/html".to_string(),
            HttpContentType::TextJavascript => "text/javascript".to_string(),
            HttpContentType::TextCSS => "text/css".to_string(),
            HttpContentType::ApplicationJson => "application/json".to_string(),
            HttpContentType::ImagePng => "image/png".to_string(),
            HttpContentType::ImageJpeg => "image/jpeg".to_string(),
            HttpContentType::ImageGif => "image/gif".to_string(),
            HttpContentType::ImageWebp => "image/webp".to_string(),
        }
    }
}


struct HttpResponse {
    status_code: u16,
    content: Option<Vec<u8>>,
    content_type: Option<HttpContentType>,
}


impl HttpResponse {
    fn new(status_code: u16, content: impl Into<Vec<u8>>, content_type: HttpContentType) -> Self {
        Self { status_code, content: Some(content.into()), content_type: Some(content_type) }
    }

    fn new_without_content(status_code: u16) -> Self {
        Self { status_code, content: None, content_type: None }
    }

    fn server_error_default_response() -> Self {
        Self::new(500, "Internal Error", HttpContentType::TextPlain)
    }

    fn server_error_with_message(msg: &str) -> Self {
        Self::new(500, format!("Internal Server Error: {}", msg), HttpContentType::TextPlain)
    }

    fn from_image_file(img_path: &Path) -> Result<Self, String> {
        let image_data = fs::read(img_path).map_err(|err| err.to_string())?;
        let content_type = match img_path.extension().and_then(|s| s.to_str()) {
            Some("png") => HttpContentType::ImagePng,
            Some("jpg") | Some("jpeg") => HttpContentType::ImageJpeg,
            Some("gif") => HttpContentType::ImageGif,
            Some("webp") => HttpContentType::ImageWebp,
            _ => return Err("Unknown image extension".to_owned())
        };

        Ok(Self {
            status_code: 200,
            content: Some(image_data),
            content_type: Some(content_type),
        })
    }
}


impl HttpResponse {
    fn response_string(self) -> Vec<u8> {
        let status_text = match self.status_code {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            // Fallback for other codes
            200..=299 => "Success",
            300..=399 => "Redirection",
            400..=499 => "Client Error",
            500..=599 => "Server Error",
            _ => "Unknown Status",
        };
        let status_response_line = format!("HTTP/1.1 {} {}", self.status_code, status_text);

        // 204 No Content must not have a body or Content-Length
        if self.status_code == 204 {
            return format!("{}\r\n\r\n", status_response_line).into_bytes();
        }

        let Some(content) = self.content else {
            error!("Status code was not 204, but response has empty content!");
            return HttpResponse::server_error_default_response().response_string();
        };
        let Some(content_type) = self.content_type else {
            error!("Status code was not 204, but response has empty content type!");
            return HttpResponse::server_error_default_response().response_string();
        };

        // All other responses include Content-Length and body
        let content_length = content.len();
        let header = format!(
            "{}\r\n\
            Content-Type: {}\r\n\
            Content-Length: {}\r\n\
            \r\n",
            status_response_line,
            content_type.to_string(),
            content_length
        );

        let mut response = header.into_bytes();
        response.extend_from_slice(content.as_slice());
        response
    }
}


pub struct NowPlayingServer {
    json_source_path: PathBuf,
    currently_playing: Option<MediaMetadata>,
    app_width: u64,
    app_height: u64,
}


impl NowPlayingServer {
    pub fn new(
        json_source_path: PathBuf,
        app_width: u64,
        app_height: u64
    ) -> Self {
        let mut server = NowPlayingServer {
            json_source_path,
            currently_playing: None,
            app_width,
            app_height
        };
        server.update_current_playing();
        server  // return
    }

    fn update_current_playing(&mut self) -> () {
        self.currently_playing = match deserialize_json_file(self.json_source_path.as_ref()) {
            Ok(metadata) => {
                match metadata.play_status.as_str() {
                    "playing" | "paused" => Some(metadata),
                    _ => None
                }
            }
            Err(e) => {
                error!("Unable to fetch currently playing metadata. Reason: {}", e);
                None
            }
        }
    }

    pub fn serve_connection(&mut self, mut stream: TcpStream) -> anyhow::Result<()> {
        let response = self.parse_and_serve_connection(&stream).unwrap_or_else(|msg| {
            error!("Internal server error: {}", msg);
            HttpResponse::server_error_default_response()
        });

        let response_string = response.response_string();
        stream.write_all(response_string.as_slice())?;
        Ok(())
    }

    /// Parse the stream's received HTTP headers.
    /// This is an intermediary between serve_connection() and serve_route(), which handles any
    /// egregious stream parsing errors (e.g. invalid HTTP request).
    fn parse_and_serve_connection(&mut self, stream: &TcpStream) -> Result<HttpResponse, String> {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map_while(|result| match result {
                Ok(line) if !line.is_empty() => Some(line),
                _ => None,
            })
            .collect();

        let request_header_line: &str = http_request
            .get(0)
            .ok_or_else(|| "No request line found.")?;

        let request_header_parts: Vec<&str> = request_header_line
            .split_whitespace()
            .collect();

        let &[http_method, route_path, http_version] = request_header_parts.as_slice() else {
            return Err(format!("Invalid HTTP request line: {}", request_header_line))
        };

        debug!("Method: {http_method}, Route: {route_path}, http version: {http_version}");
        Ok(self.serve_route(http_method, route_path))
    }

    /// Parse the route and handle the appropriate server route.
    /// Each route should handle any internal errors gracefully, perhaps including an appropriate
    /// error message explaining what the internal error was.
    fn serve_route(&mut self, http_method: &str, route_path: &str) -> HttpResponse {
        let (route_path, params) = match route_path.split_once('?') {
            Some((p, params)) => (p, Some(params)),
            None => (route_path, None)
        };

        match params {
            Some(param_str) => {
                debug!("Serving: {http_method} {route_path} [params = {param_str}]");
            },
            None => {
                debug!("Serving: {http_method} {route_path}");
            }
        }
        match (http_method, route_path) {
            ("GET", "/") => self.serve_root(),
            ("GET", "/js") => self.serve_root_js(),
            ("GET", "/css") => self.serve_root_css(),
            ("POST", "/update") => self.serve_update(),
            ("GET", "/metadata") => self.serve_metadata(),
            ("GET", "/album-art") => self.serve_album_art(),
            _ => self.serve_not_found(),
        }
    }



    ///
    /// Serve default 404 page.
    fn serve_not_found(&self) -> HttpResponse {
        HttpResponse::new(404, "404 Not Found", HttpContentType::TextPlain)
    }

    ///
    /// Serve "/" -- the main browser application's HTML layout.
    fn serve_root(&mut self) -> HttpResponse {
        let page_template = NowPlayingMainTemplate {
            app_width: self.app_width,
            app_height: self.app_height,
        };
        match page_template.render() {
            Ok(content) => HttpResponse::new(200, content, HttpContentType::TextHtml),
            Err(e) => {
                error!("Main page templating error: {e}");
                HttpResponse::server_error_with_message("Template error")
            },
        }
    }

    ///
    /// Serve "/js" -- the main browser application's JS logic.
    fn serve_root_js(&mut self) -> HttpResponse {
        let page_template = NowPlayingMainJSTemplate { };
        match page_template.render() {
            Ok(content) => HttpResponse::new(200, content, HttpContentType::TextJavascript),
            Err(e) => {
                error!("JS templating error: {e}");
                HttpResponse::server_error_with_message("Template error")
            },
        }
    }

    ///
    /// Serve "/css" -- the main browser application's JS logic.
    fn serve_root_css(&mut self) -> HttpResponse {
        let page_template = NowPlayingMainCSSTemplate { };
        match page_template.render() {
            Ok(content) => HttpResponse::new(200, content, HttpContentType::TextCSS),
            Err(e) => {
                error!("JS templating error: {e}");
                HttpResponse::server_error_with_message("Template error")
            },
        }
    }

    fn serve_update(&mut self) -> HttpResponse {
        self.update_current_playing();
        HttpResponse::new_without_content(204)
    }

    fn serve_album_art(&mut self) -> HttpResponse {
        let Some(metadata) = &self.currently_playing else {
            error!("No metadata to serve album art from.");
            return HttpResponse::new(404, "No metadata found", HttpContentType::TextPlain);
        };

        let Some(album_art_path) = metadata.find_album_art_path() else {
            error!("No album art found! Returning fallback art.");
            let default_art = load_default_album_art();
            return HttpResponse::new(200, default_art, HttpContentType::ImagePng);
        };

        HttpResponse::from_image_file(&album_art_path)
            .unwrap_or_else(
                |err| {
                    error!("Image IO error: {}", err);
                    let default_art = load_default_album_art();
                    return HttpResponse::new(200, default_art, HttpContentType::ImagePng);
                }
            )
    }

    fn serve_metadata(&mut self) -> HttpResponse {
        let page_template = match &self.currently_playing {
            None => MetadataJSONTemplate {
                status: "none".into(), song_id: "".into(), title: "".into(),
                artist: "".into(), album: "".into(), duration: 0
            },
            Some(metadata) => MetadataJSONTemplate::from(metadata)
        };

        match page_template.render() {
            Ok(content) => HttpResponse::new(200, content, HttpContentType::ApplicationJson),
            Err(e) => {
                error!("Metadata templating error: {e}");
                HttpResponse::server_error_with_message("Template error")
            },
        }
    }
}