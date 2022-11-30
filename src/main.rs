#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![warn(unused_must_use)]
#[macro_use]
extern crate nickel;
extern crate hyper;
extern crate rand;
extern crate url;

use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};

use nickel::hyper::header::Location;
use nickel::status::StatusCode;
use nickel::{HttpRouter, Mount, Nickel, StaticFilesHandler};
use rand::Rng;
use url::form_urlencoded;

fn main() {
    let server_url = "http://127.0.0.1:1111";
    let mut server = Nickel::new();
    let short_urls = Arc::new(Mutex::new(HashMap::new()));
    short_urls
        .lock()
        .unwrap()
        .insert("rust".to_string(), "https://www.rust-lang.org".to_string());

    let short_urls_clone = short_urls.clone();
    server.get(
        "/",
        middleware! {|_, response|
            let mut data = HashMap::new();
            let short_urls = short_urls_clone.lock().unwrap();
            data.insert("url_count", short_urls.len().to_string());
            return response.render("templates/index.tpl", &data);
        },
    );

    let short_urls_clone = short_urls.clone();
    server.post(
        "/shorten",
        middleware! {|request, response|
            let mut data = HashMap::new();
            let mut short_urls = short_urls_clone.lock().unwrap();
            data.insert("url_count", short_urls.len().to_string());

            let mut post_data = String::new();
            request.origin.read_to_string(&mut post_data).unwrap();
            let form = parse_form(post_data.as_bytes());

            let url = form.get("url").unwrap_or(&"".to_string()).to_string();
            let mut key = generate_emoji_set(3);
            if url != "" {
                    while short_urls.contains_key(&key) {
                        key = generate_emoji_set(3);
                    }
                    short_urls.insert(urlencoding::encode(&key.clone()).to_string(), url);
            }else {
                data.insert("result", "You need to enter a URL.".to_string());
            }
            return response.send(format!("Here is your short URL: {}/{}", server_url, &key));
        },
    );

    let short_urls_clone = short_urls.clone();
    server.get(
        "/:shortkey",
        middleware! {|request, mut response|
            let short_urls = short_urls_clone.lock().unwrap();
            let shortkey = request.param("shortkey").unwrap();
            if short_urls.contains_key(shortkey) {
                let url = short_urls.get(shortkey).unwrap();
                response.set(StatusCode::TemporaryRedirect);
                response.set(Location(url.clone()));
                return response.send("");
            }else {
                return response.send("Short URL not found");
            }
        },
    );

    //Serve static_files/ from /static/
    server.utilize(Mount::new(
        "/static/",
        StaticFilesHandler::new("static_files/"),
    ));

    server.listen("127.0.0.1:1111");
}

fn generate_emoji_set(size: usize) -> String {
    // return rand::thread_rng().gen_ascii_chars().take(size).collect()
    let mut emojis = String::new();
    for i in 0..size {
        let x: u32 = rand::thread_rng().gen_range(0x1F600, 0x1F64F);
        let emoji = char::from_u32(x).unwrap_or('💔');
        emojis.push(emoji);
    }
    return emojis.to_string();
}

fn parse_form(form_data: &[u8]) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    let parsed_form = form_urlencoded::parse(form_data);
    for (key, value) in parsed_form {
        hashmap.insert(key.to_string(), value.to_string());
    }
    return hashmap;
}
