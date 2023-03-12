use crate::grammatik::{Genus, Kasus, Numerus};

use super::StammDeklination;

pub struct ODeklinationMaskulinumFemininum<'a> {
    stamm: &'a str,
}

impl<'a> StammDeklination<'a> for ODeklinationMaskulinumFemininum<'a> {
    const DEFAULT_GENUS: Option<Genus> = Some(Genus::Maskulinum);
    const ALLOWS_MASKULINUM: bool = true;
    const ALLOWS_FEMININUM: bool = true;
    const ALLOWS_NEUTRUM: bool = false;

    fn from_stamm(stamm: &'a str) -> Self {
        Self { stamm }
    }

    fn get_stamm(&self) -> &'a str {
        self.stamm
    }

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        Some(match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ => "us",
                Kasus::Genitiv => "i",
                Kasus::Dativ => "o",
                Kasus::Akkusativ => "um",
                Kasus::Ablativ => "o",
                Kasus::Vokativ => return None,
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "i",
                Kasus::Genitiv => "orum",
                Kasus::Dativ => "is",
                Kasus::Akkusativ => "os",
                Kasus::Ablativ => "is",
            },
        })
    }

    fn get_endung_instance(&self, numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        if let (Numerus::Singular, Kasus::Vokativ) = (numerus, kasus) {
            Some(if self.stamm.ends_with('i') { "" } else { "e" })
        } else {
            None
        }
    }
}
