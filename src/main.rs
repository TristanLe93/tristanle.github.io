use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};
use std::{collections::HashMap, fs::File, io::Read};

const REGEX_STR: &str = r#"odin 2 (base|pro|max)\s(black|white|clear blue|clear purple|cold grey):\s*[od]*([0-9x]*)[- ]*[od]*([0-9x]*)"#;

fn main() {
    //let source = source_file();
    let source = source_webpage();

    let document = Html::parse_document(&source);
    let main = Selector::parse("div.rte.rte--expanded-images.clearfix.row").unwrap();
    let paragraph = Selector::parse("p").unwrap();

    let main_selector = document.select(&main).next().unwrap();

    let regex = Regex::new(REGEX_STR).unwrap();

    let mut map = HashMap::new();

    for e in main_selector.select(&paragraph) {
        let html = e.inner_html().to_lowercase();
        for str in regex.find_iter(&html) {
            let captures = regex.captures(str.as_str()).unwrap();
            let model = captures.get(1).unwrap().as_str().to_string();
            let color = captures.get(2).unwrap().as_str().to_string();
            let num = captures.get(4).unwrap().as_str().to_string();

            map.insert((model, color), num.to_string());
        }
    }

    let mut items = map
        .into_iter()
        .map(|((model, color), value)| format!("{model} {color}: {value}"))
        .collect::<Vec<_>>();

    let order = |s: &str| match s {
        "base" => 1,
        "pro" => 2,
        "max" => 3,
        _ => 4,
    };

    items.sort_by(|a, b| {
        let split_a: Vec<&str> = a.split_whitespace().collect();
        let split_b: Vec<&str> = b.split_whitespace().collect();

        let order_a = order(split_a[0]);
        let order_b = order(split_b[0]);

        match order_a.cmp(&order_b) {
            std::cmp::Ordering::Equal => split_a[1].cmp(split_b[1]),
            other => other,
        }
    });

    for i in items {
        println!("{i}");
    }
}

fn source_webpage() -> String {
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        ),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        header::HeaderValue::from_static("en-US,en;q=0.5"),
    );

    client
        .get("https://www.ayntec.com/pages/shipment-dashboard")
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn source_file() -> String {
    let mut f = File::open("dashboard.html").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    buffer
}
