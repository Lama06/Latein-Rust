use crate::grammatik::{Kasus, Numerus};

use super::Deklination;

fn get_endung(numerus: Numerus, kasus: Kasus) -> &'static str {
    match numerus {
        Numerus::Singular => match kasus {
            Kasus::Nominativ => "um",
            Kasus::Genitiv => "i",
            Kasus::Dativ => "o",
            Kasus::Akkusativ => "um",
            Kasus::Ablativ => "o",
            Kasus::Vokativ => "um",
        },
        Numerus::Plural => match kasus {
            Kasus::Nominativ => "a",
            Kasus::Genitiv => "orum",
            Kasus::Dativ => "is",
            Kasus::Akkusativ => "a",
            Kasus::Ablativ => "is",
            Kasus::Vokativ => "a",
        },
    }
}

pub struct ODeklinationNeutrum<'a> {
    stamm: &'a str,
}

impl<'a> ODeklinationNeutrum<'a> {
    pub fn new(stamm: &'a str) -> Self {
        Self { stamm }
    }
}

impl<'a> Deklination for ODeklinationNeutrum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String {
        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(get_endung(numerus, kasus));
        result
    }
}
