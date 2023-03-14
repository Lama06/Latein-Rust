use crate::grammatik::{test_form, Genus, Kasus, Numerus, Steigerung};

use super::{
    superlativ::SuperlativDeklination, Deklination, ParsableDeklination, WörterbuchEintrag,
};

fn get_adverb_endung(stamm: &str) -> &'static str {
    if stamm.ends_with("nt") {
        "er"
    } else {
        "iter"
    }
}

pub fn get_endung(genus: Genus, numerus: Numerus, kasus: Kasus) -> &'static str {
    match genus {
        Genus::Maskulinum | Genus::Femininum => match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => panic!(),
                Kasus::Genitiv => "is",
                Kasus::Dativ => "i",
                Kasus::Akkusativ => "em",
                Kasus::Ablativ => "i",
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "es",
                Kasus::Genitiv => "ium",
                Kasus::Dativ => "ibus",
                Kasus::Akkusativ => "es",
                Kasus::Ablativ => "ibus",
            },
        },
        Genus::Neutrum => match numerus {
            Numerus::Singular => match kasus {
                Kasus::Nominativ | Kasus::Vokativ | Kasus::Akkusativ => panic!(),
                Kasus::Genitiv => "is",
                Kasus::Dativ => "i",
                Kasus::Ablativ => "i",
            },
            Numerus::Plural => match kasus {
                Kasus::Nominativ | Kasus::Vokativ => "ia",
                Kasus::Genitiv => "ium",
                Kasus::Dativ => "ibus",
                Kasus::Akkusativ => "ia",
                Kasus::Ablativ => "ibus",
            },
        },
    }
}

#[derive(Clone)]
pub struct KonsonantischeDeklination<'a> {
    nominativ_singular_maskulinum: &'a str,
    nominativ_singular_femininum: &'a str,
    nominativ_singular_neutrum: &'a str,
    stamm: &'a str,
}

impl<'a> KonsonantischeDeklination<'a> {
    // vehenens, vehenentis
    fn parse_einendig(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: None,
        } = eintrag else {
            return None;
        };

        let stamm = if zweite_form.ends_with("is") {
            &zweite_form[..zweite_form.len() - 2]
        } else {
            return None;
        };

        Some(Self {
            nominativ_singular_maskulinum: erste_form,
            nominativ_singular_femininum: erste_form,
            nominativ_singular_neutrum: erste_form,
            stamm,
        })
    }

    // fortis, e
    fn parse_zweiendig_short(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: None,
        } = eintrag else {
            return None;
        };

        let stamm = if erste_form.ends_with("is") {
            &zweite_form[..erste_form.len() - 2]
        } else {
            return None;
        };

        if zweite_form != "e" {
            return None;
        }

        Some(Self {
            nominativ_singular_maskulinum: erste_form,
            nominativ_singular_femininum: erste_form,
            nominativ_singular_neutrum: zweite_form,
            stamm,
        })
    }

    // fortis, forte
    fn parse_zweiendig_long(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: None,
        } = eintrag else {
            return None;
        };

        let stamm = if erste_form.ends_with("is") {
            &zweite_form[..erste_form.len() - 2]
        } else {
            return None;
        };

        if !test_form(zweite_form, stamm, "e") {
            return None;
        }

        Some(Self {
            nominativ_singular_maskulinum: erste_form,
            nominativ_singular_femininum: erste_form,
            nominativ_singular_neutrum: zweite_form,
            stamm,
        })
    }

    // acer, acris, acre
    fn parse_dreiendig(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let &WörterbuchEintrag {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: Some(dritte_form),
        } = eintrag else {
            return None;
        };

        let stamm = if zweite_form.ends_with("is") {
            &zweite_form[..zweite_form.len() - 2]
        } else {
            return None;
        };

        if !test_form(dritte_form, stamm, "e") {
            return None;
        }

        Some(Self {
            nominativ_singular_maskulinum: erste_form,
            nominativ_singular_femininum: zweite_form,
            nominativ_singular_neutrum: dritte_form,
            stamm,
        })
    }
}

impl<'a> Deklination<'a> for KonsonantischeDeklination<'a> {
    fn adverb(&self) -> String {
        let adverb_endung = get_adverb_endung(self.stamm);
        let mut adverb = String::with_capacity(self.stamm.len() + adverb_endung.len());
        adverb.push_str(self.stamm);
        adverb.push_str(adverb_endung);
        adverb
    }

    fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        if let (Kasus::Nominativ | Kasus::Vokativ, Numerus::Singular) = (kasus, numerus) {
            return String::from(match genus {
                Genus::Maskulinum => self.nominativ_singular_maskulinum,
                Genus::Femininum => self.nominativ_singular_femininum,
                Genus::Neutrum => self.nominativ_singular_neutrum,
            });
        }

        if let (Kasus::Akkusativ, Numerus::Singular, Genus::Neutrum) = (kasus, numerus, genus) {
            return String::from(self.nominativ_singular_neutrum);
        }

        let endung = get_endung(genus, numerus, kasus);
        let mut form = String::with_capacity(self.stamm.len() + endung.len());
        form.push_str(&self.stamm);
        form.push_str(endung);
        form
    }

    fn steigern(&self, steigerung: Steigerung) -> Option<Box<dyn Deklination<'a> + 'a>> {
        Some(match steigerung {
            Steigerung::Positiv => Box::new(self.clone()),
            Steigerung::Komperativ => todo!(),
            Steigerung::Superlativ => Box::new(SuperlativDeklination::new(self.stamm)),
        })
    }
}

impl<'a> ParsableDeklination<'a> for KonsonantischeDeklination<'a> {
    fn parse(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        if let result @ Some(_) = Self::parse_einendig(eintrag) {
            result
        } else if let result @ Some(_) = Self::parse_zweiendig_short(eintrag) {
            result
        } else if let result @ Some(_) = Self::parse_zweiendig_long(eintrag) {
            result
        } else if let result @ Some(_) = Self::parse_dreiendig(eintrag) {
            result
        } else {
            None
        }
    }
}
