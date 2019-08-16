#[cfg(test)]
mod tests {
    use scraper::Html;
    use scraper::Selector;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_no_translation() {
        let document = std::fs::read_to_string("dict-responses/asddgf.html").unwrap();
        let document = Html::parse_document(&document);

        let selector = "tr[id^='tr']";
        let selector = Selector::parse(selector).unwrap();

        for e in document.select(&selector) {
            panic!("false positive for translation: {:#?}", e.value());
        }
    }

    #[test]
    fn parse_many_translations() {
        let document = std::fs::read_to_string("dict-responses/valid.html").unwrap();
        let document = Html::parse_document(&document);

        let selector = "tr[id^='tr']";
        let selector = Selector::parse(selector).unwrap();

        let mut loop_count = 0;
        for _e in document.select(&selector) {
            loop_count += 1;
        }
        assert_eq!(loop_count, 50);
    }

    #[test]
    fn parse_no_results_but_suggestions() {
        let document = std::fs::read_to_string("dict-responses/mispelt.html").unwrap();
        let document = Html::parse_document(&document);

        let selector = "tr[id^='tr']";
        let selector = Selector::parse(selector).unwrap();

        for e in document.select(&selector) {
            panic!("false positive for translation: {:#?}", e.value());
        }
    }
}
