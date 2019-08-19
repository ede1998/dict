mod dictcc;
mod language;

pub type Translations = Vec<(String, String)>;
pub type Suggestions = (Vec<String>, Vec<String>);

pub enum Entries {
    Translation(Translations),
    Suggestion(Suggestions),
    NotSet,
    NoResultsFound,
}

pub trait Translator {
    fn translate(&mut self, request: &str);
    fn entries(&self) -> &Entries;
}

pub use dictcc::DictccTranslator;
pub use language::Language;
