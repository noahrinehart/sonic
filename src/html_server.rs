use std::path::Path;
use db;
use nickel::{HttpRouter, MediaType, MiddlewareResult, Nickel, Options, Request, Response};

pub fn html_middleware<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let path = Path::new("public/index.html");
    res.send_file(path)
}

pub fn api_middleware(room: &str) -> String {
    let col = db::get_database_col();
    let messages = db::fetch_messages_from_room(&col, &room);
    let mut data_result = "{\"messages\":[".to_owned();
    for (i, message) in messages.iter().enumerate() {
        let mut message_json = format!(
            "{{
            \"message\":\"{}\",
            \"room\":\"{}\",
            \"encrypted\":\"{}\"
            ,\"created_at\":\"{}\"}}",
            message.message,
            message.room,
            message.encrypted,
            message.created_at.unwrap().to_rfc3339()
        );
        if i != messages.iter().len() - 1 {
            message_json.push_str(",");
        }
        data_result.push_str(&message_json);
    }
    data_result.push_str("]}");
    data_result
}

pub fn new_server() -> Nickel {
    let mut html = Nickel::new();
    html.options = Options::default().output_on_listen(false);
    html.get("/*", html_middleware);
    html.get(
        "/api/:room",
        middleware! { |req, mut res|
            let room = req.param("room").unwrap();
            let api_result = api_middleware(&room);
            res.set(MediaType::Json);
            format!("{}", api_result)
        },
    );
    html
}
