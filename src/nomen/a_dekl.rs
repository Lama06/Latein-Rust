use crate::grammatik::{Kasus, Numerus};

use super::Deklination;

fn get_endung(numerus: Numerus, kasus: Kasus) -> &'static str {
    match numerus {
        Numerus::Singular => match kasus {
            Kasus::Nominativ | Kasus::Vokativ => "a",
            Kasus::Genitiv => "ae",
            Kasus::Dativ => "ae",
            Kasus::Akkusativ => "am",
            Kasus::Ablativ => "a",
        },
        Numerus::Plural => match kasus {
            Kasus::Nominativ | Kasus::Vokativ => "ae",
            Kasus::Genitiv => "arum",
            Kasus::Dativ => "is",
            Kasus::Akkusativ => "as",
            Kasus::Ablativ => "is",
        },
    }
}

pub struct ADeklination<'a> {
    stamm: &'a str,
}

impl<'a> ADeklination<'a> {
    pub fn new(stamm: &'a str) -> Self {
        Self { stamm }
    }
}

impl<'a> Deklination for ADeklination<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String {
        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(get_endung(numerus, kasus));
        result
    }
}
