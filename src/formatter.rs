use crate::translation_extractor::Entries;

// TODO should be added to Entries
pub fn print(entries: &Entries) {
    use Entries::*;
    match entries {
        Translation(t) => print_translations(&t),
        Suggestion(_s) => panic!("Not implemented"),
        NotSet => println!("No search done yet."),
    }
}

fn print_translations(translations: &Vec<(String, String)>) {
    const MINIMUM_GAP: usize = 8;
    const FILLER_CHAR: &str = ".";

    let longest = translations
        .iter()
        .map(|pair| pair.0.len())
        .max()
        .expect("No translations in Vec")
        + MINIMUM_GAP;

    for (l, r) in translations {
        let filler = longest - l.chars().count();
        let filler: String = String::from(FILLER_CHAR).repeat(filler);
        println!("{}{}{}", l, filler, r);
    }
}
