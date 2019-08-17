pub enum Entries {
    Translation(Vec<(String, String)>),
    Suggestion(Vec<String>),
    NotSet,
}

pub trait Translator {
    fn translate(&mut self, request: &str);
    fn entries(&self) -> &Entries;
}

pub struct DictccTranslator {
    entries: Entries,
}

impl DictccTranslator {
    pub fn new() -> DictccTranslator {
        DictccTranslator {
            entries: Entries::NotSet,
        }
    }

    fn download_translations(_request: &str) -> String {
        reqwest::get("https://dict.cc?s=test")
            .unwrap()
            .text()
            .unwrap()
    }

    fn parse_column(html: &str, column_selector: &str) -> Vec<String> {
        use scraper::{ElementRef, Html, Selector};

        let document = Html::parse_document(&html);
        let selector = Selector::parse(column_selector).unwrap();

        let rows: Vec<String> = document
            .select(&selector)
            .map(|element| {
                let mut content = String::new();

                // concatenate string from desired node texts
                for node in element.children() {
                    match ElementRef::wrap(node) {
                        Some(node) => {
                            let undesired_tags = vec!["dfn", "div"];
                            if !undesired_tags.contains(&node.value().name()) {
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
            })
            .collect();

        rows
    }

    fn parse_translations(&mut self, html: &str) {
        const LEFT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(2)";
        const RIGHT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(3)";

        let left = DictccTranslator::parse_column(html, LEFT_SELECTOR);
        let right = DictccTranslator::parse_column(html, RIGHT_SELECTOR);
        assert_eq!(left.len(), right.len());
        assert!(!left.is_empty());

        let left = left.into_iter();
        let right = right.into_iter();

        let result = left.zip(right).collect();

        self.entries = Entries::Translation(result);
    }
}

impl Translator for DictccTranslator {
    fn translate(&mut self, request: &str) {
        let html = DictccTranslator::download_translations(request);
        self.parse_translations(&html);
    }

    fn entries(&self) -> &Entries {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    fn read_translations(filename: &str) -> Vec<(String, String)> {
        use crate::itertools::Itertools;
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut translations = Vec::new();

        // extract translation pairs
        for line in reader.lines() {
            let line = line.unwrap();

            let pair: (&str, &str) = line.split("=>").next_tuple().expect("Invalid file format");
            let pair = (pair.0.to_string(), pair.1.to_string());
            translations.push(pair);
        }

        translations
    }

    struct TestFixture {
        document_asddgf: String,
        document_valid: String,
        solutions_valid: Vec<(String, String)>,
    }

    lazy_static! {
        static ref FIXTURE: TestFixture = TestFixture {
            document_asddgf: std::fs::read_to_string("dict-responses/asddgf.html").unwrap(),
            document_valid: std::fs::read_to_string("dict-responses/valid.html").unwrap(),
            solutions_valid: read_translations("dict-responses/valid.tl"),
        };
    }

    #[test]
    fn empty_column() {
        let document = std::fs::read_to_string("dict-responses/asddgf.html").unwrap();
        const LEFT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(2)";

        let entries = DictccTranslator::parse_column(&document, LEFT_SELECTOR);

        assert!(entries.is_empty());
    }

    #[test]
    fn full_column() {
        let document = std::fs::read_to_string("dict-responses/valid.html").unwrap();
        const LEFT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(2)";

        let entries = DictccTranslator::parse_column(&document, LEFT_SELECTOR);

        assert_eq!(entries.len(), 50);
    }

    #[test]
    fn many_translations() {
        let document = std::fs::read_to_string("dict-responses/valid.html").unwrap();
        let mut translator = DictccTranslator::new();

        translator.parse_translations(&document);

        match translator.entries() {
            Entries::Translation(translations) => {
                assert_eq!(translations.len(), 50);
                assert_eq!(translations, &FIXTURE.solutions_valid);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn no_translation() {}

    //#[test]
    //fn parse_no_results_but_suggestions() {
    //    let document = std::fs::read_to_string("dict-responses/mispelt.html").unwrap();
    //    let document = Html::parse_document(&document);

    //    let selector = "tr[id^='tr']";
    //    let selector = Selector::parse(selector).unwrap();

    //    for e in document.select(&selector) {
    //        panic!("false positive for translation: {:#?}", e.value());
    //    }
    //}
}
