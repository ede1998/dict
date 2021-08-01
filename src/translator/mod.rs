mod dictcc;
mod language;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Translations(Vec<(String, String)>);

impl std::ops::Deref for Translations {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Translations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(String, String)> for Translations {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let t = iter.into_iter().collect();
        Self(t)
    }
}

impl From<Vec<(String, String)>> for Translations {
    fn from(f: Vec<(String, String)>) -> Self {
        Self(f)
    }
}

pub type Suggestions = (Vec<String>, Vec<String>);

#[derive(Debug)]
pub enum Entries {
    Translation(Translations),
    Suggestion(Suggestions),
    NotSet,
    NoResultsFound,
}

pub trait Translator {
    fn translate(&mut self, request: &str) {
        self.set_query(request);
        self.translate_query();
    }
    fn translate_query(&mut self);
    fn entries(&self) -> &Entries;
    fn query(&self) -> &str;
    fn set_query(&mut self, query: &str);

    fn languages(&self) -> LanguagePair;
    fn set_languages(&mut self, language: LanguagePair);
    fn set_languages_if_available(&mut self, languages: LanguagePair) -> bool;

    fn is_language_available(language: LanguagePair) -> bool;
    fn get_available_languages() -> Vec<LanguagePair>;
}

use std::iter::FromIterator;

pub use dictcc::DictccTranslator;
pub use language::Language;
pub use language::LanguagePair;
pub use language::DEFAULT_LANGUAGES;
