use crate::scanner::results::HttpResponse;
use std::io::prelude::*;
use std::time::Duration;
use std::thread;

pub fn send_request(url: &String, path: &String, delay: Duration) -> Result<HttpResponse, reqwest::Error> {
    let mut res = reqwest::blocking::get(url)?;

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    thread::sleep(delay);

    Ok(HttpResponse {
        url: url.to_string(),
        path: path.to_string(),
        response: res.status(),
        length: body.len(),
    })
}