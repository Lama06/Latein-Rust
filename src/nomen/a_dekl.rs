use crate::grammatik::{Genus, Kasus, Numerus};

use super::StammDeklination;

pub struct ADeklination<'a> {
    stamm: &'a str,
    plural: bool,
}

impl<'a> StammDeklination<'a> for ADeklination<'a> {
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
        })
    }
}
