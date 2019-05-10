extern crate easy_http_request;

use easy_http_request::StaticHttpRequest;
use std::collections::HashMap;

fn main() {
    let mut request = StaticHttpRequest::get_from_url_str("http://127.0.0.1:8000/time").unwrap();

    request.headers = Some({
        let mut map = HashMap::with_capacity(1);

        map.insert("Authorization", "WVdCXmcO07VdKX8GA");

        map
    });

    match request.send() {
        Ok(response) => match response.status_code {
            401 => eprintln!("The auth key is wrong!"),
            200 => println!("{}", String::from_utf8(response.body).unwrap()),
            _ => eprintln!("Unknown error. The status code is {}.", response.status_code),
        }
        Err(_) => eprintln!("Please run api.rs first. Use `cargo run --example get_system_time_api`.")
    }
}