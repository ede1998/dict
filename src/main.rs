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
        print_info(subarguments);
    } else {
        issue_query(&arguments);
    }
}

fn issue_query(arguments: &clap::ArgMatches) {
    let mut language = (Language::DE, Language::EN);
    if let Some(mut pair) = arguments.get_many(args::LANGUAGE_PAIR) {
        language = (
            *pair.next().expect("getting first language identifier"),
            *pair.next().expect("getting second language identifier"),
        );

        assert_eq!(None, pair.next());
    }

    let mut translator = DictccTranslator::new();
    if !translator.set_languages_if_available(language) {
        println!("Language pair not available.");
        std::process::exit(0);
    }

    println!("{} >>>>>>>>>>>>>>>>>>> {}", language.0, language.1);

    let query: &String = arguments.get_one(args::QUERY).unwrap();
    translator.translate(query);
    print(translator);
}

fn print_info(arguments: &clap::ArgMatches) {
    let subcommand = arguments
        .subcommand_name()
        .expect("Unexpected missing subcommand. Please contact the developer.");
    match subcommand {
        args::AVAILABLE => {
            println!("The following language pairs are available:");
            for (l1, l2) in DictccTranslator::get_available_languages() {
                println!(
                    "{} {} => {} - {}",
                    l1.get_abbreviation(),
                    l2.get_abbreviation(),
                    l1,
                    l2
                );
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
