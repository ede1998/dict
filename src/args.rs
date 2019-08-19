use clap::{Arg, ArgMatches};

use crate::translator;
use translator::Language;
use translator::DEFAULT_LANGUAGES;

pub const LANGUAGE_PAIR: &str = "language pairs";
pub const QUERY: &str = "query word";

pub fn parse() -> ArgMatches<'static> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name(LANGUAGE_PAIR)
                .short("l")
                .long("languages")
                .number_of_values(2)
                .help(&format!(
                    "Languages to translate between (default: {}-{})",
                    DEFAULT_LANGUAGES.0.get_abbreviation(),
                    DEFAULT_LANGUAGES.1.get_abbreviation()
                ))
                .takes_value(true)
                .case_insensitive(true)
                .possible_values(&Language::get_all_language_abbreviations()[..]),
        )
        .arg(
            Arg::with_name(QUERY)
                .help("The word that should be translated")
                .required(true)
                .index(1),
        )
        .get_matches();

    matches
}