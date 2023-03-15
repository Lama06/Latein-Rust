use crate::grammatik::{Genus, Kasus, Numerus, Steigerung};

use super::{ao_dekl::get_endung, Deklination};

pub struct KomperativDeklination<'a> {
    positiv_stamm: &'a str,
}

impl<'a> KomperativDeklination<'a> {
    pub fn new(positiv_stamm: &'a str) -> Self {
        Self { positiv_stamm }
    }

    pub fn adverb(&self) -> String {
        const ENDUNG: &'static str = "ius";
        let mut adverb = String::with_capacity(self.positiv_stamm.len() + ENDUNG.len());
        adverb.push_str(self.positiv_stamm);
        adverb.push_str(ENDUNG);
        adverb
    }

    pub fn deklinieren(&self, genus: Genus, numerus: Numerus, kasus: Kasus) -> String {
        const STAMM_ERWEITERUNG: &'static str = "ior";
        const STAMM_ERWEITERUNG_NEUTRUM: &'static str = "ius";

        match (genus, numerus, kasus) {
            (
                Genus::Maskulinum | Genus::Femininum,
                Numerus::Singular,
                Kasus::Nominativ | Kasus::Vokativ,
            ) => {
                let mut form =
                    String::with_capacity(self.positiv_stamm.len() + STAMM_ERWEITERUNG.len());
                form.push_str(self.positiv_stamm);
                form.push_str(STAMM_ERWEITERUNG);
                return form;
            }
            (Genus::Neutrum, Numerus::Singular, Kasus::Nominativ | Kasus::Akkusativ) => {
                let mut form = String::with_capacity(
                    self.positiv_stamm.len() + STAMM_ERWEITERUNG_NEUTRUM.len(),
                );
                form.push_str(self.positiv_stamm);
                form.push_str(STAMM_ERWEITERUNG_NEUTRUM);
                return form;
            }
            _ => (),
        }

        let endung = get_endung(genus, numerus, kasus);
        let mut form = String::with_capacity(
            self.positiv_stamm.len() + STAMM_ERWEITERUNG.len() + endung.len(),
        );
        form.push_str(self.positiv_stamm);
        form.push_str(STAMM_ERWEITERUNG);
        form.push_str(endung);
        form
    }

    pub(super) fn steigern(&self, _: Steigerung) -> Option<Deklination<'static>> {
        None
    }
}
