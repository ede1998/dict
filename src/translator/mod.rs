mod dictcc;
mod language;

pub enum Entries {
    Translation(Vec<(String, String)>),
    Suggestion((Vec<String>, Vec<String>)),
    NotSet,
    NoResultsFound,
}

pub trait Translator {
    fn translate(&mut self, request: &str);
    fn entries(&self) -> &Entries;
}

pub use dictcc::DictccTranslator;
pub use language::Language;
