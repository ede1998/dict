use crate::translator::{Entries, Suggestions, Translations, Translator};

pub fn print(translator: impl Translator) {
    use Entries::*;
    match translator.entries() {
        Translation(t) => print_translations(t),
        Suggestion(s) => print_suggestions(s),
        NotSet => println!("No search done yet."),
        NoResultsFound => println!("No results found."),
    }
}

fn print_translations(translations: &Translations) {
    const MINIMUM_GAP: usize = 8;
    const FILLER_CHAR: &str = ".";

    let longest = translations
        .iter()
        .map(|pair| pair.0.len())
        .max()
        .expect("No translations in Vec")
        + MINIMUM_GAP;

    println!("========== TRANSLATIONS ==========");

    for (l, r) in translations.iter() {
        let filler = longest - l.chars().count();
        let filler: String = String::from(FILLER_CHAR).repeat(filler);
        println!("{} {} {}", l, filler, r);
    }
}

fn print_suggestions(suggestions: &Suggestions) {
    use itertools::EitherOrBoth;
    use itertools::Itertools;
    const MINIMUM_GAP: usize = 8;
    const FILLER_CHAR: &str = " ";

    let longest = suggestions
        .0
        .iter()
        .map(|element| element.len())
        .max()
        .unwrap_or(0)
        + MINIMUM_GAP;

    let left = suggestions.0.iter();
    let right = suggestions.1.iter();

    println!("No perfect matches found.");
    println!("========== SUGGESTIONS ==========");

    for either_or_both in left.zip_longest(right) {
        match either_or_both {
            EitherOrBoth::Both(l, r) => {
                let filler = longest - l.chars().count();
                let filler: String = String::from(FILLER_CHAR).repeat(filler);
                println!("{} {} {}", l, filler, r);
            }
            EitherOrBoth::Left(l) => println!("{}", l),
            EitherOrBoth::Right(r) => {
                let filler = String::from(FILLER_CHAR).repeat(longest);
                println!(" {} {}", filler, r);
            }
        }
    }
}
