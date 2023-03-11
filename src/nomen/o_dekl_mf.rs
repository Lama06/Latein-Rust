use crate::grammatik::{Kasus, Numerus};

use super::Deklination;

fn get_endung(stamm: &str, numerus: Numerus, kasus: Kasus) -> &'static str {
    match numerus {
        Numerus::Singular => match kasus {
            Kasus::Nominativ => "us",
            Kasus::Genitiv => "i",
            Kasus::Dativ => "o",
            Kasus::Akkusativ => "um",
            Kasus::Ablativ => "o",
            Kasus::Vokativ => {
                if stamm.ends_with('i') {
                    "i"
                } else {
                    "e"
                }
            }
        },
        Numerus::Plural => match kasus {
            Kasus::Nominativ => "i",
            Kasus::Genitiv => "orum",
            Kasus::Dativ => "is",
            Kasus::Akkusativ => "os",
            Kasus::Ablativ => "is",
            Kasus::Vokativ => "i",
        },
    }
}

pub struct ODeklinationMaskulinumFemininum<'a> {
    stamm: &'a str,
}

impl<'a> ODeklinationMaskulinumFemininum<'a> {
    pub fn new(stamm: &'a str) -> Self {
        Self { stamm }
    }
}

impl<'a> Deklination for ODeklinationMaskulinumFemininum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String {
        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(get_endung(self.stamm, numerus, kasus));
        result
    }
}
