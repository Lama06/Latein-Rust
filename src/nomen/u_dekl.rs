use crate::grammatik::{Genus, Kasus, Numerus};

use super::StammDeklination;

pub struct UDeklination<'a> {
    stamm: &'a str,
}

impl<'a> StammDeklination<'a> for UDeklination<'a> {
    const DEFAULT_GENUS: Option<Genus> = Some(Genus::Maskulinum);
    const ALLOWS_MASKULINUM: bool = true;
    const ALLOWS_FEMININUM: bool = true;
    const ALLOWS_NEUTRUM: bool = false;

    const REQUIRE_GENITIVE: bool = true;

    fn from_stamm(stamm: &'a str) -> Self {
        Self { stamm }
    }

    fn get_stamm(&self) -> &'a str {
        self.stamm
    }

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        Some(match numerus {
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
        })
    }
}
