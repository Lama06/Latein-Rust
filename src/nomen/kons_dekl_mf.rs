use crate::grammatik::{Genus, Kasus, Numerus};

use super::{test_form, Deklination, ParsableDeklination};

pub struct KonsonantischeDeklinationMaskulinumFemininum<'a> {
    nominativ_singular: Option<&'a str>,
    stamm: &'a str,
    plural: bool,
}

impl<'a> Deklination for KonsonantischeDeklinationMaskulinumFemininum<'a> {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        if self.plural && matches!(numerus, Numerus::Singular) {
            return None;
        }

        let endung = match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => {
                    return Some(String::from(self.nominativ_singular.unwrap()))
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

        let mut result = String::with_capacity(self.stamm.len() + endung.len());
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

        if genitiv.ends_with("is") {
            Some(Self {
                nominativ_singular: Some(nominativ),
                stamm: &genitiv[..genitiv.len() - 2],
                plural: false,
            })
        } else if genitiv.ends_with("um") {
            let stamm = &genitiv[..genitiv.len() - 2];
            if test_form(nominativ, stamm, "es") {
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
