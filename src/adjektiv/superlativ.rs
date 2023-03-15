use crate::grammatik::{Genus, Kasus, Numerus};

use super::ao_dekl::{get_endung, ADVERB_ENDUNG};

const STAMM_ERWEITERUNG: &'static str = "issim";

// TODO: pulcher, pulchra, pulchrum -> pulchrissimus, a, um
pub struct SuperlativDeklination<'a> {
    positiv_stamm: &'a str,
}

impl<'a> SuperlativDeklination<'a> {
    pub fn new(positiv_stamm: &'a str) -> Self {
        Self { positiv_stamm }
    }

    pub fn adverb(&self) -> String {
        let mut adverb = String::with_capacity(
            self.positiv_stamm.len() + STAMM_ERWEITERUNG.len() + ADVERB_ENDUNG.len(),
        );
        adverb.push_str(self.positiv_stamm);
        adverb.push_str(STAMM_ERWEITERUNG);
        adverb.push_str(ADVERB_ENDUNG);
        adverb
    }

    pub fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        let endung = get_endung(genus, numerus, kasus);
        let mut form = String::with_capacity(
            self.positiv_stamm.len() + STAMM_ERWEITERUNG.len() + endung.len(),
        );
        form.push_str(self.positiv_stamm);
        form.push_str(STAMM_ERWEITERUNG);
        form.push_str(endung);
        form
    }
}
