use askama::Template;


#[derive(Template)]
#[template(path = "main.html", ext = "html")]
pub struct NowPlayingMainTemplate {
    pub app_width: u64,
    pub app_height: u64
}


#[derive(Template)]
#[template(path = "main.js", ext = "js", escape = "none")]
pub struct NowPlayingMainJSTemplate {
}


#[derive(Template)]
#[template(path = "main.css", ext = "css", escape = "none")]
pub struct NowPlayingMainCSSTemplate {
}

