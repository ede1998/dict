extern crate itertools;
extern crate reqwest;
#[cfg(test)]
extern crate lazy_static;
mod translation_extractor;

use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;

fn main() {
    let body = reqwest::get("https://dict.cc?s=test")
        .unwrap()
        .text()
        .unwrap();

    //println!("body = {:?}", body);

    let document = Html::parse_document(&body);

    //println!("{:#?}", document);

    let left_selector = "tr[id^='tr'] > :nth-child(2)";
    let left_selector = Selector::parse(left_selector).unwrap();

    let right_selector = "tr[id^='tr'] > :nth-child(3)";
    let _right_selector = Selector::parse(right_selector).unwrap();

    //let mut pairs = Vec::new();
    let pairs: Vec<String> = document
        .select(&left_selector)
        .map(|element| {
            let mut content = String::new();
            for node in element.children() {
                match ElementRef::wrap(node) {
                    Some(node) => {
                        if node.value().name() != "dfn" {
                            content.push_str(&node.text().collect::<String>());
                        }
                    }
                    None => {
                        if let Some(node) = node.value().as_text() {
                            content.push_str(&node);
                        }
                    }
                }
            }
            String::from(content.trim())
        }).collect();
    println!("{:#?}", pairs);
}
