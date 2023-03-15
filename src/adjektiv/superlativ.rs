use crate::grammatik::{Genus, Kasus, Numerus, Steigerung};

use super::{
    ao_dekl::{get_endung, ADVERB_ENDUNG},
    Deklination,
};

pub struct SuperlativDeklination<'a> {
    positiv_stamm: &'a str,
}

impl<'a> SuperlativDeklination<'a> {
    pub fn new(positiv_stamm: &'a str) -> Self {
        Self { positiv_stamm }
    }

    pub fn adverb(&self) -> String {
        let stamm_erweiterung = "issim";
        let mut adverb = String::with_capacity(
            self.positiv_stamm.len() + stamm_erweiterung.len() + ADVERB_ENDUNG.len(),
        );
        adverb.push_str(self.positiv_stamm);
        adverb.push_str(stamm_erweiterung);
        adverb.push_str(ADVERB_ENDUNG);
        adverb
    }

    pub fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        let stamm_erweiterung = "issim";
        let endung = get_endung(genus, numerus, kasus);
        let mut form = String::with_capacity(
            self.positiv_stamm.len() + stamm_erweiterung.len() + endung.len(),
        );
        form.push_str(self.positiv_stamm);
        form.push_str(stamm_erweiterung);
        form.push_str(endung);
        form
    }

    pub(super) fn steigern(&self, _: Steigerung) -> Option<Deklination<'static>> {
        None
    }
}
