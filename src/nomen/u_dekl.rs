use crate::grammatik::{Kasus, Numerus};

use super::Deklination;

fn get_endung(numerus: Numerus, kasus: Kasus) -> &'static str {
    match numerus {
        Numerus::Singular => match kasus {
            Kasus::Nominativ | Kasus::Vokativ => "us",
            Kasus::Genitiv => "us",
            Kasus::Dativ => "ui",
            Kasus::Akkusativ => "um",
            Kasus::Ablativ => "u",
        },
        Numerus::Plural => match kasus {
            Kasus::Nominativ | Kasus::Vokativ => "us",
            Kasus::Genitiv => "uum",
            Kasus::Dativ => "ibus",
            Kasus::Akkusativ => "us",
            Kasus::Ablativ => "ibus",
        },
    }
}

pub struct UDeklination<'a> {
    stamm: &'a str,
}

impl<'a> UDeklination<'a> {
    pub fn new(stamm: &'a str) -> Self {
        Self { stamm }
    }
}

impl<'a> Deklination for UDeklination<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String {
        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(get_endung(numerus, kasus));
        result
    }
}
