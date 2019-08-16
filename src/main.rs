extern crate itertools;
extern crate reqwest;
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

    let right_selector = "tr[id^='tr'] > :nth-child(2)";
    let right_selector = Selector::parse(right_selector).unwrap();

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
    
    for line in pairs {
        println!("{}", line.trim());
    }
    return;

    //let mut pairs = Vec::new();
    let pairs: Vec<String> = document
        .select(&left_selector)
        .map(|element| {
            let selector = Selector::parse("td > *").expect("HERE");
            element.select(&selector).map(|elem| {
                if elem.value().name() == "dfn" {
                    return String::from("TREASON");
                }
                elem.text().collect()
            }).collect()
        }) // concatenate all text nodes
         .collect();

    //let pairs:Vec<ElementRef> = document.select(&left_selector).collect();


    println!("{:#?}", pairs);
    //for t in &pairs {
    //    for c in t.children() {
    //        println!("{}", ElementRef::wrap(c).unwrap().inner_html());
    //    }
    //}
    //println!("{}", &pairs.len());

}
