use crate::grammatik::{Kasus, Numerus};

use super::Deklination;

fn get_endung(numerus: Numerus, kasus: Kasus) -> &'static str {
    match numerus {
        Numerus::Singular => match kasus {
            Kasus::Nominativ | Kasus::Vokativ => "es",
            Kasus::Genitiv => "ei",
            Kasus::Dativ => "ei",
            Kasus::Akkusativ => "em",
            Kasus::Ablativ => "e",
        },
        Numerus::Plural => match kasus {
            Kasus::Nominativ | Kasus::Vokativ => "es",
            Kasus::Genitiv => "erum",
            Kasus::Dativ => "ebus",
            Kasus::Akkusativ => "es",
            Kasus::Ablativ => "ebus",
        },
    }
}

pub struct EDeklination<'a> {
    stamm: &'a str,
}

impl<'a> EDeklination<'a> {
    pub fn new(stamm: &'a str) -> Self {
        Self { stamm }
    }
}

impl<'a> Deklination for EDeklination<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String {
        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(get_endung(numerus, kasus));
        result
    }
}
