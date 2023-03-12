use crate::grammatik::{Genus, Kasus, Numerus};

use super::{Deklination, ParsableDeklination, PluralDeklination, StammDeklination};

pub struct KonsonantischeDeklinationNeutrum<'a> {
    nominativ_singular: &'a str,
    stamm: &'a str,
}

impl<'a> Deklination for KonsonantischeDeklinationNeutrum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        let endung = match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ | Kasus::Akkusativ => {
                    return Some(String::from(self.nominativ_singular))
                }
                Kasus::Genitiv => "is",
                Kasus::Dativ => "i",
                Kasus::Ablativ => "e",
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "a",
                Kasus::Genitiv => "um",
                Kasus::Dativ => "ibus",
                Kasus::Akkusativ => "a",
                Kasus::Ablativ => "ibus",
            },
        };

        let mut result = String::new();
        result.push_str(self.stamm);
        result.push_str(endung);
        Some(result)
    }
}

impl<'a> ParsableDeklination<'a> for KonsonantischeDeklinationNeutrum<'a> {
    const DEFAULT_GENUS: Option<Genus> = None;
    const ALLOWS_MASKULINUM: bool = false;
    const ALLOWS_FEMININUM: bool = false;
    const ALLOWS_NEUTRUM: bool = true;

    fn parse_wörterbuch_formen(nominativ: &'a str, genitiv: Option<&'a str>) -> Option<Self> {
        let Some(genitiv) = genitiv else {
            return None;
        };

        let stamm = if genitiv.ends_with("is") {
            &genitiv[..genitiv.len() - 2]
        } else {
            return None;
        };

        Some(Self {
            nominativ_singular: nominativ,
            stamm,
        })
    }
}

impl<'a> StammDeklination<'a> for PluralDeklination<KonsonantischeDeklinationNeutrum<'a>> {
    const ALLOWS_MASKULINUM: bool = false;
    const ALLOWS_FEMININUM: bool = false;
    const ALLOWS_NEUTRUM: bool = true;
    const DEFAULT_GENUS: Option<Genus> = None;

    const PLURAL: bool = true;
    const REQUIRE_GENITIVE: bool = true;

    fn from_stamm(stamm: &'a str) -> Self {
        PluralDeklination(KonsonantischeDeklinationNeutrum { nominativ_singular: "", stamm })
    }

    fn get_stamm(&self) -> &str {
        self.0.stamm
    }

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        Some(match numerus {
            Numerus::Singular => unreachable!(),
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "a",
                Kasus::Genitiv => "um",
                Kasus::Dativ => "ibus",
                Kasus::Akkusativ => "a",
                Kasus::Ablativ => "ibus",
            },
        })
    }
}