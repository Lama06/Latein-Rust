use crate::grammatik::{Genus, Kasus, Numerus};

use super::{Deklination, ParsableDeklination, PluralDeklination, StammDeklination};

pub struct KonsonantischeDeklinationMaskulinumFemininum<'a> {
    nominativ_singular: &'a str,
    stamm: &'a str,
}

impl<'a> Deklination for KonsonantischeDeklinationMaskulinumFemininum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        let endung = match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => {
                    return Some(String::from(self.nominativ_singular))
                }
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
        Some(result)
    }
}

impl<'a> ParsableDeklination<'a> for KonsonantischeDeklinationMaskulinumFemininum<'a> {
    const DEFAULT_GENUS: Option<Genus> = None;
    const ALLOWS_FEMININUM: bool = true;
    const ALLOWS_MASKULINUM: bool = true;
    const ALLOWS_NEUTRUM: bool = false;

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

impl<'a> StammDeklination<'a>
    for PluralDeklination<KonsonantischeDeklinationMaskulinumFemininum<'a>>
{
    const ALLOWS_MASKULINUM: bool = true;
    const ALLOWS_FEMININUM: bool = true;
    const ALLOWS_NEUTRUM: bool = false;
    const DEFAULT_GENUS: Option<Genus> = None;

    const PLURAL: bool = true;
    const REQUIRE_GENITIVE: bool = true;

    fn from_stamm(stamm: &'a str) -> Self {
        PluralDeklination(KonsonantischeDeklinationMaskulinumFemininum {
            nominativ_singular: "",
            stamm,
        })
    }

    fn get_stamm(&self) -> &str {
        self.0.stamm
    }

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        Some(match numerus {
            Numerus::Singular => unreachable!(),
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "es",
                Kasus::Genitiv => "um",
                Kasus::Dativ => "ibus",
                Kasus::Akkusativ => "es",
                Kasus::Ablativ => "ibus",
            },
        })
    }
}
