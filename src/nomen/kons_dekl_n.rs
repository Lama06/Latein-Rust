use crate::grammatik::{test_form, Genus, Kasus, Numerus};

use super::{Deklination, ParsableDeklination};

pub struct KonsonantischeDeklinationNeutrum<'a> {
    nominativ_singular: Option<&'a str>,
    stamm: &'a str,
    plural: bool,
}

impl<'a> Deklination for KonsonantischeDeklinationNeutrum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        if self.plural && matches!(numerus, Numerus::Singular) {
            return None;
        }

        let endung = match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ | Kasus::Akkusativ => {
                    return Some(String::from(self.nominativ_singular.unwrap()))
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

        let mut result = String::with_capacity(self.stamm.len() + endung.len());
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

        if genitiv.ends_with("is") {
            Some(Self {
                nominativ_singular: Some(nominativ),
                stamm: &genitiv[..genitiv.len() - 2],
                plural: false,
            })
        } else if genitiv.ends_with("um") {
            let stamm = &genitiv[..genitiv.len() - 2];
            if test_form(nominativ, stamm, "a") {
                Some(Self {
                    nominativ_singular: None,
                    stamm,
                    plural: true,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
