use crate::grammatik::{Genus, Kasus, Numerus, Steigerung};

use self::{
    ao_dekl::AODeklination, komperativ::KomperativDeklination,
    kons_dekl::KonsonantischeDeklination, superlativ::SuperlativDeklination,
};

mod ao_dekl;
mod komperativ;
mod kons_dekl;
mod superlativ;

#[cfg(test)]
mod tests;

enum Deklination<'a> {
    Ao(AODeklination<'a>),
    Konsonantische(KonsonantischeDeklination<'a>),
    Komperativ(KomperativDeklination<'a>),
    Superlativ(SuperlativDeklination<'a>),
}

impl<'a> Deklination<'a> {
    fn parse(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        if let Some(deklination) = AODeklination::parse(eintrag) {
            Some(Self::Ao(deklination))
        } else if let Some(deklination) = KonsonantischeDeklination::parse(eintrag) {
            Some(Self::Konsonantische(deklination))
        } else {
            None
        }
    }

    fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        match *self {
            Self::Ao(ref deklination) => deklination.deklinieren(genus, numerus, kasus),
            Self::Konsonantische(ref deklination) => deklination.deklinieren(genus, numerus, kasus),
            Self::Komperativ(ref deklination) => deklination.deklinieren(genus, numerus, kasus),
            Self::Superlativ(ref deklination) => deklination.deklinieren(genus, numerus, kasus),
        }
    }

    fn adverb(&self) -> String {
        match *self {
            Self::Ao(ref deklination) => deklination.adverb(),
            Self::Konsonantische(ref deklination) => deklination.adverb(),
            Self::Komperativ(ref deklination) => deklination.adverb(),
            Self::Superlativ(ref deklination) => deklination.adverb(),
        }
    }

    fn steigern(&self, steigerung: Steigerung) -> Option<Deklination<'a>> {
        match *self {
            Self::Ao(ref deklination) => deklination.steigern(steigerung),
            Self::Konsonantische(ref deklination) => deklination.steigern(steigerung),
            Self::Komperativ(ref deklination) => deklination.steigern(steigerung),
            Self::Superlativ(ref deklination) => deklination.steigern(steigerung),
        }
    }
}

#[derive(Debug)]
pub struct WörterbuchEintrag<'a> {
    erste_form: &'a str,
    zweite_form: Option<&'a str>,
    dritte_form: Option<&'a str>,
}

impl<'a> WörterbuchEintrag<'a> {
    pub fn from_one(erste_form: &'a str) -> Self {
        Self {
            erste_form,
            zweite_form: None,
            dritte_form: None,
        }
    }

    pub fn from_two(erste_form: &'a str, zweite_form: &'a str) -> Self {
        Self {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: None,
        }
    }

    pub fn from_three(erste_form: &'a str, zweite_form: &'a str, dritte_form: &'a str) -> Self {
        Self {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: Some(dritte_form),
        }
    }
}

pub struct Adjektiv<'a> {
    deklination: Deklination<'a>,
}

impl<'a> Adjektiv<'a> {
    pub fn parse(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let Some(deklination) = Deklination::parse(eintrag) else {
            return None;
        };

        Some(Self { deklination })
    }

    pub fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        self.deklination.deklinieren(genus, numerus, kasus)
    }

    pub fn steigern(&self, steigerung: Steigerung) -> Option<Self> {
        Some(Self {
            deklination: match self.deklination.steigern(steigerung) {
                Some(deklination) => deklination,
                None => return None,
            },
        })
    }

    pub fn adverb(&self) -> String {
        self.deklination.adverb()
    }
}
