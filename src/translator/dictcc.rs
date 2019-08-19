use scraper::Html;
use std::fmt;

use super::*;

pub struct DictccTranslator {
    entries: Entries,
}

#[derive(Debug)]
enum RequestError {
    UrlError(reqwest::UrlError),
    DownloadError(reqwest::Error),
}

impl From<reqwest::UrlError> for RequestError {
    fn from(error: reqwest::UrlError) -> Self {
        RequestError::UrlError(error)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        RequestError::DownloadError(error)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::UrlError(e) => fmt::Display::fmt(e, f),
            RequestError::DownloadError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl DictccTranslator {
    pub fn new() -> Self {
        DictccTranslator {
            entries: Entries::NotSet,
        }
    }

    fn download_translations(request: &str) -> Result<String, RequestError> {
        const URL: &str = "https://dict.cc";
        let request = reqwest::Url::parse_with_params(URL, &[("s", request)])?;
        Ok(reqwest::get(request)?.text()?)
    }

    fn parse_column(html: &Html, column_selector: &str) -> Vec<String> {
        use scraper::{ElementRef, Selector};

        let selector = Selector::parse(column_selector).unwrap();

        let rows: Vec<String> = html
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

    fn parse_translations(html: &Html) -> Option<Translations> {
        const LEFT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(2)";
        const RIGHT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(3)";

        let left = DictccTranslator::parse_column(html, LEFT_SELECTOR);
        let right = DictccTranslator::parse_column(html, RIGHT_SELECTOR);
        assert_eq!(left.len(), right.len());

        if left.is_empty() {
            return None;
        }

        let left = left.into_iter();
        let right = right.into_iter();

        let result = left.zip(right).collect();

        Some(result)
    }

    fn parse_suggestions(html: &Html) -> Option<Suggestions> {
        const LEFT_SELECTOR: &str = "td.td3nl:first-of-type > a";
        const RIGHT_SELECTOR: &str = "td.td3nl:last-of-type > a";

        let left = DictccTranslator::parse_column(&html, LEFT_SELECTOR);
        let right = DictccTranslator::parse_column(&html, RIGHT_SELECTOR);

        if left.is_empty() && right.is_empty() {
            return None;
        }

        Some((left, right))
    }
}

impl Translator for DictccTranslator {
    fn translate(&mut self, request: &str) {
        match DictccTranslator::download_translations(request) {
            Ok(html) => {
                let document = Html::parse_document(&html);
                match DictccTranslator::parse_translations(&document) {
                    Some(t) => self.entries = Entries::Translation(t),
                    None => match DictccTranslator::parse_suggestions(&document) {
                        Some(s) => self.entries = Entries::Suggestion(s),
                        None => self.entries = Entries::NoResultsFound,
                    },
                }
            }
            Err(failure) => println!(
                "Requesting translations from dict.cc failed. Reason: {}",
                failure
            ),
        }
    }

    fn entries(&self) -> &Entries {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_translations(filename: &str) -> Translations {
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

    fn read_suggestions(filename: &str) -> Suggestions {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut suggestions = (Vec::new(), Vec::new());
        let mut before_swap = true;

        for line in reader.lines() {
            let line = line.unwrap();

            if line == "" {
                before_swap = false;
                continue;
            }

            if before_swap {
                suggestions.0.push(line);
            } else {
                suggestions.1.push(line);
            }
        }

        suggestions
    }

    fn read_website(filename: &str) -> Html {
        let document = std::fs::read_to_string(filename).unwrap();

        Html::parse_document(&document)
    }

    #[test]
    fn column_empty() {
        let challenge = read_website("dict-responses/asddgf.html");
        const LEFT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(2)";

        let entries = DictccTranslator::parse_column(&challenge, LEFT_SELECTOR);

        assert!(entries.is_empty());
    }

    #[test]
    fn column_full() {
        let challenge = read_website("dict-responses/valid.html");
        let solution = read_translations("dict-responses/valid.tl");
        const LEFT_SELECTOR: &str = "tr[id^='tr'] > :nth-child(2)";

        let entries = DictccTranslator::parse_column(&challenge, LEFT_SELECTOR);

        assert_eq!(entries.len(), 50);
        itertools::assert_equal(&entries, solution.iter().map(|tuple| &tuple.0));
    }

    #[test]
    fn translations_many() {
        let challenge = read_website("dict-responses/valid.html");
        let solution = read_translations("dict-responses/valid.tl");

        let result = DictccTranslator::parse_translations(&challenge);

        let result = result.unwrap();
        assert_eq!(result.len(), 50);
        assert_eq!(&result, &solution);
    }

    #[test]
    fn translations_none() {
        let challenge = read_website("dict-responses/mispelt.html");

        let result = DictccTranslator::parse_translations(&challenge);

        assert!(result.is_none());
    }

    #[test]
    fn suggestions_many() {
        let challenge = read_website("dict-responses/mispelt.html");
        let solution = read_suggestions("dict-responses/mispelt.sugg");

        let result = DictccTranslator::parse_suggestions(&challenge);

        let result = result.unwrap();
        assert_eq!(&result, &solution);
    }

    #[test]
    fn suggestions_none() {
        let challenge = read_website("dict-responses/asddgf.html");

        let result = DictccTranslator::parse_translations(&challenge);
        assert!(result.is_none());
        let result = DictccTranslator::parse_suggestions(&challenge);
        assert!(result.is_none());
    }

    #[test]
    fn download() {
        const WORD: &str = "test";

        let result = DictccTranslator::download_translations(WORD);

        let result = result.unwrap();
        Html::parse_document(&result);
    }
}
