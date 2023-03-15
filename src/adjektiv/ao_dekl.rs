use crate::grammatik::{test_form, Genus, Kasus, Numerus, Steigerung};

use super::{
    komperativ::KomperativDeklination, superlativ::SuperlativDeklination, Deklination,
    WörterbuchEintrag,
};

pub const ADVERB_ENDUNG: &'static str = "e";

pub fn get_endung(genus: Genus, numerus: Numerus, kasus: Kasus) -> &'static str {
    match genus {
        Genus::Maskulinum => match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ => "us",
                Kasus::Genitiv => "i",
                Kasus::Dativ => "o",
                Kasus::Akkusativ => "um",
                Kasus::Ablativ => "o",
                Kasus::Vokativ => "e",
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "i",
                Kasus::Genitiv => "orum",
                Kasus::Dativ => "is",
                Kasus::Akkusativ => "os",
                Kasus::Ablativ => "is",
            },
        },
        Genus::Femininum => match numerus {
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
        },
        Genus::Neutrum => match numerus {
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
        },
    }
}

#[derive(Clone)]
pub struct AODeklination<'a> {
    nominativ_singular_maskulinum: Option<&'a str>,
    stamm: &'a str,
}

impl<'a> AODeklination<'a> {
    // bonus
    fn parse_a_um_single(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: None,
            dritte_form: None,
        } = eintrag else {
            return None;
        };

        let stamm = if erste_form.ends_with("us") {
            &erste_form[..erste_form.len() - 2]
        } else {
            return None;
        };

        Some(Self {
            nominativ_singular_maskulinum: None,
            stamm,
        })
    }

    // bonus, bona, bonum
    fn parse_a_um_long(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: Some(dritte_form),
        } = eintrag else {
            return None;
        };

        let stamm = if erste_form.ends_with("us") {
            &erste_form[..erste_form.len() - 2]
        } else {
            return None;
        };

        if !test_form(zweite_form, stamm, "a") || !test_form(dritte_form, stamm, "um") {
            return None;
        }

        Some(Self {
            nominativ_singular_maskulinum: None,
            stamm,
        })
    }

    // bonus, a, um
    fn parse_a_um_short(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some("a"),
            dritte_form: Some("um"),
        } = eintrag else {
            return None;
        };

        let stamm = if erste_form.ends_with("us") {
            &erste_form[..erste_form.len() - 2]
        } else {
            return None;
        };

        Some(Self {
            nominativ_singular_maskulinum: None,
            stamm,
        })
    }

    // pulcher, pulchra, pulchrum
    fn parse_er(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: Some(dritte_form),
        } = eintrag else {
            return None;
        };

        if !erste_form.ends_with("er") {
            return None;
        }

        let stamm = if zweite_form.ends_with("a") {
            &zweite_form[..zweite_form.len() - 1]
        } else {
            return None;
        };

        if !test_form(dritte_form, stamm, "um") {
            return None;
        }

        return Some(Self {
            nominativ_singular_maskulinum: Some(erste_form),
            stamm,
        });
    }

    pub fn parse(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        if let result @ Some(_) = Self::parse_a_um_single(eintrag) {
            result
        } else if let result @ Some(_) = Self::parse_a_um_short(eintrag) {
            result
        } else if let result @ Some(_) = Self::parse_a_um_long(eintrag) {
            result
        } else if let result @ Some(_) = Self::parse_er(eintrag) {
            result
        } else {
            None
        }
    }

    pub fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        if let Some(nominativ_singular_maskulinum) = self.nominativ_singular_maskulinum {
            if let (Kasus::Nominativ, Numerus::Singular, Genus::Maskulinum) =
                (kasus, numerus, genus)
            {
                return String::from(nominativ_singular_maskulinum);
            }
        }

        let endung = get_endung(genus, numerus, kasus);
        let mut form = String::with_capacity(self.stamm.len() + endung.len());
        form.push_str(self.stamm);
        form.push_str(endung);
        form
    }

    pub fn adverb(&self) -> String {
        let mut adverb = String::with_capacity(self.stamm.len() + ADVERB_ENDUNG.len());
        adverb.push_str(self.stamm);
        adverb.push_str(ADVERB_ENDUNG);
        adverb
    }

    pub(super) fn steigern(&self, steigerung: Steigerung) -> Option<Deklination<'a>> {
        Some(match steigerung {
            Steigerung::Positiv => Deklination::Ao(self.clone()),
            Steigerung::Komperativ => {
                Deklination::Komperativ(KomperativDeklination::new(self.stamm))
            }
            Steigerung::Superlativ => {
                Deklination::Superlativ(SuperlativDeklination::new(self.stamm))
            }
        })
    }
}
