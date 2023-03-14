use crate::grammatik::{Genus, Kasus, Numerus, Steigerung};

use self::{ao_dekl::AODeklination, kons_dekl::KonsonantischeDeklination};

mod ao_dekl;
mod komperativ;
mod kons_dekl;
mod superlativ;

#[cfg(test)]
mod tests;

trait Deklination<'a> {
    fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String;

    fn adverb(&self) -> String;

    fn steigern(&self, steigerung: Steigerung) -> Option<Box<dyn Deklination<'a> + 'a>>;
}

trait ParsableDeklination<'a>: Sized + Deklination<'a> {
    fn parse(eintrag: &WörterbuchEintrag<'a>) -> Option<Self>;
}

#[derive(Debug)]
pub struct WörterbuchEintrag<'a> {
    erste_form: &'a str,
    zweite_form: Option<&'a str>,
    dritte_form: Option<&'a str>,
}

impl<'a> WörterbuchEintrag<'a> {
    fn from_one(erste_form: &'a str) -> Self {
        Self {
            erste_form,
            zweite_form: None,
            dritte_form: None,
        }
    }

    fn from_two(erste_form: &'a str, zweite_form: &'a str) -> Self {
        Self {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: None,
        }
    }

    fn from_three(erste_form: &'a str, zweite_form: &'a str, dritte_form: &'a str) -> Self {
        Self {
            erste_form,
            zweite_form: Some(zweite_form),
            dritte_form: Some(dritte_form),
        }
    }

    fn parse_deklination<T>(&self) -> Option<Box<dyn Deklination<'a> + 'a>>
    where
        T: ParsableDeklination<'a> + 'a,
    {
        match T::parse(self) {
            Some(deklination) => Some(Box::new(deklination)),
            None => None,
        }
    }

    fn parse(&self) -> Option<Box<dyn Deklination<'a> + 'a>> {
        if let result @ Some(_) = self.parse_deklination::<AODeklination>() {
            result
        } else if let result @ Some(_) = self.parse_deklination::<KonsonantischeDeklination>() {
            result
        } else {
            None
        }
    }
}

pub struct Adjektiv<'a> {
    deklination: Box<dyn Deklination<'a> + 'a>,
}

impl<'a> Adjektiv<'a> {
    pub fn parse(eintrag: &WörterbuchEintrag<'a>) -> Option<Self> {
        let Some(deklination) = eintrag.parse() else {
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
