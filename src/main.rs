#[macro_use(
    app_from_crate,
    crate_name,
    crate_version,
    crate_authors,
    crate_description
    )]
extern crate clap;
extern crate itertools;
extern crate reqwest;

mod args;
mod formatter;
mod translator;

use translator::DictccTranslator;
use translator::Language;
use translator::Translator;

use formatter::print;

fn main() {
    let arguments = args::parse();

    if let Some(subarguments) = arguments.subcommand_matches(args::INFO) {
        print_info(&subarguments);
    } else {
        issue_query(&arguments);
    }
}

fn issue_query(arguments: &clap::ArgMatches) {
    let mut language = (Language::DE, Language::EN);
    if let Some(pair) = arguments.values_of(args::LANGUAGE_PAIR) {
        let pair: Vec<&str> = pair.collect();
        assert_eq!(pair.len(), 2);
        language = (
            pair[0].parse::<Language>().unwrap(),
            pair[1].parse::<Language>().unwrap(),
            );
    }

    let mut translator = DictccTranslator::new();
    if !translator.set_languages_if_available(language) {
        println!("Language pair not available.");
        std::process::exit(0);
    }

    println!("{} >>>>>>>>>>>>>>>>>>> {}", language.0, language.1);

    let query = arguments.value_of(args::QUERY).unwrap();
    translator.translate(&query);
    print(translator);
}

fn print_info(arguments: &clap::ArgMatches) {
    let subcommand = arguments.subcommand_name().expect("Unexpected missing subcommand. Please contact the developer.");
    match subcommand {
        args::AVAILABLE => {
            let languages = Language::get_all_languages();
            println!("The following language pairs are available:");
            for l1 in [Language::EN, Language::DE].iter() {
                for l2 in &languages {
                    if DictccTranslator::is_language_available((*l1,*l2)) {
                        println!("{} {} => {} - {}", l1.get_abbreviation(), l2.get_abbreviation(), l1, l2);
                    }
                }
                println!();
            }
        }
        args::ABBREVIATIONS => {
            let mut languages = Language::get_all_languages();
            languages.sort_by_key(|l| l.to_string());

            println!("The following abbreviations exist (sorted by full language name):");
            for lang in languages {
                println!("{} => {}", lang.get_abbreviation(), lang);
            }
        }
        _ => panic!("Unexpected missing subcommand. Please contact the developer."),
    }
}
