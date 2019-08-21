use clap::{AppSettings, Arg, ArgMatches, SubCommand};

use crate::translator;
use translator::Language;
use translator::DEFAULT_LANGUAGES;

pub const LANGUAGE_PAIR: &str = "language pairs";
pub const QUERY: &str = "query word";
pub const INFO: &str = "info";
pub const AVAILABLE: &str = "available";
pub const ABBREVIATIONS: &str = "abbreviations";

pub fn parse() -> ArgMatches<'static> {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
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
        .subcommand(
            SubCommand::with_name(INFO)
                .about("prints more information about languages")
                .arg(
                    Arg::with_name(AVAILABLE)
                        .help("prints all available language pairs")
                        .conflicts_with(ABBREVIATIONS),
                )
                .arg(
                    Arg::with_name(ABBREVIATIONS)
                        .help("prints the long names and abbreviations for each language")
                        .conflicts_with(AVAILABLE),
                ),
        )
        .get_matches();

    matches
}
