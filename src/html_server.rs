use std::path::Path;
use std::thread;
use std::thread::{JoinHandle};
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, Options};

pub fn html_middleware<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let path = Path::new("public/index.html");
    res.send_file(path)
}

pub fn api_middleware<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let room = req.param("room").unwrap();
    res.send(format!("Room is {}", room))
}

pub fn new_server() -> Nickel {
    let mut html = Nickel::new();
    html.options = Options::default().output_on_listen(false);
    html.get("/*", html_middleware);
    html.get("/api/:room", api_middleware);
    html
}