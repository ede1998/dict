use clap::Command;
use clap::builder::EnumValueParser;
use clap::{Arg, ArgMatches};

use crate::translator;
use translator::Language;
use translator::DEFAULT_LANGUAGES;

pub const LANGUAGE_PAIR: &str = "language pairs";
pub const QUERY: &str = "query word";
pub const INFO: &str = "info";
pub const AVAILABLE: &str = "available";
pub const ABBREVIATIONS: &str = "abbreviations";

pub fn parse() -> ArgMatches {
    let matches = clap::command!()
        .subcommand_negates_reqs(true)
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new(LANGUAGE_PAIR)
                .short('l')
                .long("languages")
                .number_of_values(2)
                .help(
                    format!(
                        "Languages to translate between (default: {}-{})",
                        DEFAULT_LANGUAGES.0.get_abbreviation(),
                        DEFAULT_LANGUAGES.1.get_abbreviation()
                    )
                )
                //.takes_value(true)
                .ignore_case(true)
                .value_parser(EnumValueParser::<Language>::new()),
        )
        .arg(
            Arg::new(QUERY)
                .help("The word that should be translated")
                .required(true)
                .index(1),
        )
        .subcommand(
            Command::new(INFO)
                .about("prints more information about languages")
                .disable_help_subcommand(true)
                .arg_required_else_help(true)
                .subcommand_required(true)
                .subcommand(
                    Command::new(AVAILABLE)
                        .about("prints all available language pairs")
                        .disable_help_flag(true),
                )
                .subcommand(
                    Command::new(ABBREVIATIONS)
                        .about("prints the long names and abbreviations for each language")
                        .disable_help_flag(true),
                ),
        )
        .get_matches();

    matches
}
