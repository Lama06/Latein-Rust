use crate::grammatik::{Genus, Kasus, Numerus};

use super::StammDeklination;

pub struct ODeklinationNeutrum<'a> {
    stamm: &'a str,
    plural: bool,
}

impl<'a> StammDeklination<'a> for ODeklinationNeutrum<'a> {
    const DEFAULT_GENUS: Option<Genus> = Some(Genus::Neutrum);
    const ALLOWS_MASKULINUM: bool = false;
    const ALLOWS_FEMININUM: bool = false;
    const ALLOWS_NEUTRUM: bool = true;

    fn new(stamm: &'a str, plural: bool) -> Self {
        Self { stamm, plural }
    }

    fn get_stamm(&self) -> &'a str {
        self.stamm
    }

    fn is_plural(&self) -> bool {
        self.plural
    }

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        Some(match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "um",
                Kasus::Genitiv => "i",
                Kasus::Dativ => "o",
                Kasus::Akkusativ => "um",
                Kasus::Ablativ => "o",
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "a",
                Kasus::Genitiv => "orum",
                Kasus::Dativ => "is",
                Kasus::Akkusativ => "a",
                Kasus::Ablativ => "is",
            },
        })
    }
}
