mod dictcc;
mod language;

pub type Translations = Vec<(String, String)>;
pub type Suggestions = (Vec<String>, Vec<String>);

#[derive(Debug)]
pub enum Entries {
    Translation(Translations),
    Suggestion(Suggestions),
    NotSet,
    NoResultsFound,
}

pub trait Translator {
    fn translate(&mut self, request: &str);
    fn entries(&self) -> &Entries;

    fn languages(&self) -> LanguagePair;
    fn set_languages(&mut self, language: LanguagePair);
    fn set_languages_if_available(&mut self, languages: LanguagePair) -> bool;

    fn is_language_available(language: LanguagePair) -> bool;
}

pub use dictcc::DictccTranslator;
pub use language::Language;
pub use language::LanguagePair;
pub use language::DEFAULT_LANGUAGES;
