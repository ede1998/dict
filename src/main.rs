extern crate itertools;
extern crate reqwest;
#[cfg(test)]
extern crate lazy_static;

mod translation_extractor;
mod formatter;

use translation_extractor::DictccTranslator;
use translation_extractor::Translator;

use formatter::print;

fn main() {
    let mut translator = DictccTranslator::new();
    translator.translate("lasdasfa");
    print(translator.entries());
}
