pub type LanguagePair = (Language, Language);
pub const DEFAULT_LANGUAGES: LanguagePair = (Language::EN, Language::DE);

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Language {
    BG,
    BS,
    CS,
    DA,
    DE,
    EL,
    EN,
    EO,
    ES,
    FI,
    FR,
    HR,
    HU,
    IS,
    IT,
    LA,
    NL,
    NO,
    PL,
    PT,
    RO,
    RU,
    SK,
    SQ,
    SR,
    SV,
    TR,
}

impl Language {
    pub fn get_all_languages() -> Vec<Language> {
        let mut abbrs = Vec::new();
        for l in &LANGUAGE {
            abbrs.push(l.value);
        }
        abbrs
    }

    pub fn get_abbreviation(&self) -> &str {
        let output = LANGUAGE[*self as usize];
        assert_eq!(output.value, *self);
        output.abbreviation
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct LanguageFull {
    abbreviation: &'static str,
    name: &'static str,
    value: Language,
}

const LANGUAGE: [LanguageFull;27] = [
    LanguageFull{ value: Language::BG, abbreviation: "BG", name: "Bulgarian"},
    LanguageFull{ value: Language::BS, abbreviation: "BS", name: "Bosnian"},
    LanguageFull{ value: Language::CS, abbreviation: "CS", name: "Czech"},
    LanguageFull{ value: Language::DA, abbreviation: "DA", name: "Danish"},
    LanguageFull{ value: Language::DE, abbreviation: "DE", name: "German"},
    LanguageFull{ value: Language::EL, abbreviation: "EL", name: "Greek"},
    LanguageFull{ value: Language::EN, abbreviation: "EN", name: "English"},
    LanguageFull{ value: Language::EO, abbreviation: "EO", name: "Esperanto"},
    LanguageFull{ value: Language::ES, abbreviation: "ES", name: "Spanish"},
    LanguageFull{ value: Language::FI, abbreviation: "FI", name: "Finnish"},
    LanguageFull{ value: Language::FR, abbreviation: "FR", name: "French"},
    LanguageFull{ value: Language::HR, abbreviation: "HR", name: "Croatian"},
    LanguageFull{ value: Language::HU, abbreviation: "HU", name: "Hungarian"},
    LanguageFull{ value: Language::IS, abbreviation: "IS", name: "Icelandic"},
    LanguageFull{ value: Language::IT, abbreviation: "IT", name: "Italian"},
    LanguageFull{ value: Language::LA, abbreviation: "LA", name: "Latin"},
    LanguageFull{ value: Language::NL, abbreviation: "NL", name: "Dutch"},
    LanguageFull{ value: Language::NO, abbreviation: "NO", name: "Norwegian"},
    LanguageFull{ value: Language::PL, abbreviation: "PL", name: "Polish"},
    LanguageFull{ value: Language::PT, abbreviation: "PT", name: "Portuguese"},
    LanguageFull{ value: Language::RO, abbreviation: "RO", name: "Romanian"},
    LanguageFull{ value: Language::RU, abbreviation: "RU", name: "Russian"},
    LanguageFull{ value: Language::SK, abbreviation: "SK", name: "Slovak"},
    LanguageFull{ value: Language::SQ, abbreviation: "SQ", name: "Albanian"},
    LanguageFull{ value: Language::SR, abbreviation: "SR", name: "Serbian"},
    LanguageFull{ value: Language::SV, abbreviation: "SV", name: "Swedish"},
    LanguageFull{ value: Language::TR, abbreviation: "TR", name: "Turkish"},
];

use std::fmt;

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = LANGUAGE[*self as usize];
        assert_eq!(output.value, *self);
        write!(f, "{}", output.name)
    }
}

impl std::str::FromStr for Language {

    type Err = ();

    fn from_str(s: &str) -> Result<Language, ()> {
        let s = s.trim().to_lowercase();
        for l in &LANGUAGE {
            let abbr = l.abbreviation.to_string().to_lowercase();
            let name = l.name.to_string().to_lowercase();

            if s == abbr || s == name {
                return Ok(l.value);
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_matches_with_string() {
        for l in LANGUAGE.iter().enumerate() {
            assert_eq!(l.0, ((l.1).value as usize));
        }
    }

    #[test]
    fn convert_language_from_name() {
        for l in &LANGUAGE {
            let name = l.name;
            let parsed_value = name.parse::<Language>().unwrap();
            assert_eq!(parsed_value, l.value);
        }
    }

    #[test]
    fn convert_language_from_abbr() {
        for l in &LANGUAGE {
            let abbr = format!("{:?}", l.value);
            let parsed_value = abbr.parse::<Language>().unwrap();
            assert_eq!(parsed_value, l.value);
        }
    }

}
