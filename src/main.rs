extern crate itertools;
#[cfg(test)]
extern crate lazy_static;
extern crate reqwest;

mod formatter;
mod translation_extractor;

use translation_extractor::DictccTranslator;
use translation_extractor::Translator;

use formatter::print;

fn main() {
    let mut translator = DictccTranslator::new();

    for argument in std::env::args().skip(1) {
        translator.translate(&argument);
        print(translator.entries());
    }
}
