#[macro_use(app_from_crate,crate_name,crate_version,crate_authors,crate_description)]
extern crate clap;
extern crate itertools;
extern crate reqwest;

mod formatter;
mod translator;
mod args;

use translator::DictccTranslator;
use translator::Translator;
use translator::Language;

use formatter::print;

fn main() {
    let arguments = args::parse();

    let mut language = (Language::DE, Language::EN);
    if let Some(pair) = arguments.values_of(args::LANGUAGE_PAIR) {
        let pair: Vec<&str> = pair.collect();
        assert_eq!(pair.len(), 2);
        language = (pair[0].parse::<Language>().unwrap(),
                    pair[1].parse::<Language>().unwrap());
    }

    println!("{} >>>>>>>>>>>>>>>>>>> {}", language.0, language.1);

    let mut translator = DictccTranslator::new();

    let query = arguments.value_of(args::QUERY).unwrap();
    translator.translate(&query);
    print(translator.entries());
}

