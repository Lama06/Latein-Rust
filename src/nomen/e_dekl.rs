use crate::grammatik::{Genus, Kasus, Numerus};

use super::StammDeklination;

pub struct EDeklination<'a> {
    stamm: &'a str,
    plural: bool,
}

impl<'a> StammDeklination<'a> for EDeklination<'a> {
    const DEFAULT_GENUS: Option<Genus> = Some(Genus::Femininum);
    const ALLOWS_FEMININUM: bool = true;
    const ALLOWS_MASKULINUM: bool = true;
    const ALLOWS_NEUTRUM: bool = false;

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
        })
    }
}
