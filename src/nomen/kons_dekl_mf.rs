use crate::grammatik::{Kasus, Numerus};

use super::Deklination;

pub struct KonsonantischeDeklinationMaskulinumFemininum<'a> {
    nominativ_singular: &'a str,
    stamm: &'a str,
}

impl<'a> KonsonantischeDeklinationMaskulinumFemininum<'a> {
    pub fn new(nominativ_singular: &'a str, stamm: &'a str) -> Self {
        Self {
            nominativ_singular,
            stamm,
        }
    }
}

impl<'a> Deklination for KonsonantischeDeklinationMaskulinumFemininum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String {
        let endung = match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => return String::from(self.nominativ_singular),
                Kasus::Genitiv => "is",
                Kasus::Dativ => "i",
                Kasus::Akkusativ => "em",
                Kasus::Ablativ => "e",
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "es",
                Kasus::Genitiv => "um",
                Kasus::Dativ => "ibus",
                Kasus::Akkusativ => "es",
                Kasus::Ablativ => "ibus",
            },
        };

        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(endung);
        result
    }
}
