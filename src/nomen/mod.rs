use std::collections::HashMap;

use crate::grammatik::{Genus, Kasus, Numerus};

use self::{
    a_dekl::ADeklination, e_dekl::EDeklination,
    kons_dekl_mf::KonsonantischeDeklinationMaskulinumFemininum,
    kons_dekl_n::KonsonantischeDeklinationNeutrum, o_dekl_mf::ODeklinationMaskulinumFemininum,
    o_dekl_n::ODeklinationNeutrum, u_dekl::UDeklination,
};

mod a_dekl;
mod e_dekl;
mod kons_dekl_mf;
mod kons_dekl_n;
mod o_dekl_mf;
mod o_dekl_n;
mod u_dekl;

trait Deklination {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> String;
}

fn test_form(form: &str, stamm: &str, endung: &str) -> bool {
    form.starts_with(stamm) && form.ends_with(endung) && form.len() == stamm.len() + endung.len()
}

pub struct NomenDaten<'a> {
    pub nominativ_singular: &'a str,
    pub genitiv_singular: Option<&'a str>,
    pub genus: Option<Genus>,
}

impl<'a> NomenDaten<'a> {
    fn parse_o_dekl_mf(&self) -> Option<(Genus, ODeklinationMaskulinumFemininum<'a>)> {
        let stamm = if self.nominativ_singular.ends_with("us") {
            &self.nominativ_singular[..self.nominativ_singular.len() - 2]
        } else {
            return None;
        };

        if let Some(genitiv_singular) = self.genitiv_singular {
            if !test_form(genitiv_singular, stamm, "i") {
                return None;
            }
        }

        let genus = match self.genus {
            None => Genus::Maskulinum,
            Some(genus @ (Genus::Maskulinum | Genus::Femininum)) => genus,
            Some(Genus::Neutrum) => return None,
        };

        Some((genus, ODeklinationMaskulinumFemininum::new(stamm)))
    }

    fn parse_o_dekl_n(&self) -> Option<ODeklinationNeutrum<'a>> {
        let stamm = if self.nominativ_singular.ends_with("um") {
            &self.nominativ_singular[..self.nominativ_singular.len() - 2]
        } else {
            return None;
        };

        if let Some(genitiv_singular) = self.genitiv_singular {
            if !test_form(genitiv_singular, stamm, "i") {
                return None;
            }
        }

        if let Some(Genus::Maskulinum | Genus::Femininum) = self.genus {
            return None;
        }

        Some(ODeklinationNeutrum::new(stamm))
    }

    fn parse_a_dekl(&self) -> Option<(Genus, ADeklination<'a>)> {
        let stamm = if self.nominativ_singular.ends_with("a") {
            &self.nominativ_singular[..self.nominativ_singular.len() - 1]
        } else {
            return None;
        };

        if let Some(genitiv_singular) = self.genitiv_singular {
            if !test_form(genitiv_singular, stamm, "ae") {
                return None;
            }
        }

        let genus = match self.genus {
            None => Genus::Femininum,
            Some(genus @ (Genus::Maskulinum | Genus::Femininum)) => genus,
            Some(Genus::Neutrum) => return None,
        };

        Some((genus, ADeklination::new(stamm)))
    }

    fn parse_kons_dekl_mf(
        &self,
    ) -> Option<(Genus, KonsonantischeDeklinationMaskulinumFemininum<'a>)> {
        let stamm = match self.genitiv_singular {
            Some(genitiv_singular) if genitiv_singular.ends_with("is") => {
                &genitiv_singular[..genitiv_singular.len() - 2]
            }
            Some(_) | None => return None,
        };

        let genus = match self.genus {
            Some(genus @ (Genus::Maskulinum | Genus::Femininum)) => genus,
            Some(Genus::Neutrum) => return None,
            None => return None,
        };

        Some((
            genus,
            KonsonantischeDeklinationMaskulinumFemininum::new(self.nominativ_singular, stamm),
        ))
    }

    fn parse_kons_dekl_n(&self) -> Option<KonsonantischeDeklinationNeutrum<'a>> {
        let stamm = match self.genitiv_singular {
            Some(genitiv_singular) if genitiv_singular.ends_with("is") => {
                &genitiv_singular[..genitiv_singular.len() - 2]
            }
            Some(_) | None => return None,
        };

        if let Some(Genus::Maskulinum | Genus::Femininum) | None = self.genus {
            return None;
        }

        Some(KonsonantischeDeklinationNeutrum::new(
            self.nominativ_singular,
            stamm,
        ))
    }

    fn parse_e_dekl(&self) -> Option<(Genus, EDeklination<'a>)> {
        let stamm = if self.nominativ_singular.ends_with("es") {
            &self.nominativ_singular[..self.nominativ_singular.len() - 2]
        } else {
            return None;
        };

        if let Some(genitiv_singular) = self.genitiv_singular {
            if !test_form(genitiv_singular, stamm, "ei") {
                return None;
            }
        }

        let genus = match self.genus {
            None => Genus::Femininum,
            Some(genus @ (Genus::Maskulinum | Genus::Femininum)) => genus,
            Some(Genus::Neutrum) => return None,
        };

        Some((genus, EDeklination::new(stamm)))
    }

    fn parse_u_dekl(&self) -> Option<(Genus, UDeklination<'a>)> {
        let stamm = if self.nominativ_singular.ends_with("us") {
            &self.nominativ_singular[..self.nominativ_singular.len() - 2]
        } else {
            return None;
        };

        if let Some(genitiv_singular) = self.genitiv_singular {
            if !test_form(genitiv_singular, stamm, "us") {
                return None;
            }
        }

        let genus = match self.genus {
            None => Genus::Maskulinum,
            Some(genus @ (Genus::Maskulinum | Genus::Femininum)) => genus,
            Some(Genus::Neutrum) => return None,
        };

        Some((genus, UDeklination::new(stamm)))
    }

    fn parse(&self) -> Option<(Genus, Box<dyn Deklination + 'a>)> {
        if let Some((genus, deklination)) = self.parse_o_dekl_mf() {
            return Some((genus, Box::new(deklination)));
        }

        if let Some(deklination) = self.parse_o_dekl_n() {
            return Some((Genus::Neutrum, Box::new(deklination)));
        }

        if let Some((genus, deklination)) = self.parse_a_dekl() {
            return Some((genus, Box::new(deklination)));
        }

        if let Some((genus, deklination)) = self.parse_kons_dekl_mf() {
            return Some((genus, Box::new(deklination)));
        }

        if let Some(deklination) = self.parse_kons_dekl_n() {
            return Some((Genus::Neutrum, Box::new(deklination)));
        }

        if let Some((genus, deklination)) = self.parse_e_dekl() {
            return Some((genus, Box::new(deklination)));
        }

        if let Some((genus, deklination)) = self.parse_u_dekl() {
            return Some((genus, Box::new(deklination)));
        }

        None
    }
}

pub struct Nomen {
    genus: Genus,
    formen: HashMap<(Numerus, Kasus), String>,
}

impl Nomen {
    pub fn parse(daten: NomenDaten) -> Option<Self> {
        let (genus, deklination) = match daten.parse() {
            Some(result) => result,
            None => return None,
        };

        Some(Self {
            genus,
            formen: {
                let mut formen = HashMap::new();

                for numerus in Numerus::ALLE {
                    for kasus in Kasus::ALLE {
                        formen.insert((numerus, kasus), deklination.deklinieren(numerus, kasus));
                    }
                }

                formen
            },
        })
    }

    pub fn get_genus(&self) -> Genus {
        self.genus
    }

    pub fn deklinieren<'a>(&'a self, numerus: Numerus, kasus: Kasus) -> &'a str {
        match self.formen.get(&(numerus, kasus)) {
            Some(form) => form,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Genus::{Femininum as F, Maskulinum as M, Neutrum as N},
        Kasus::{
            Ablativ as Abl, Akkusativ as Akk, Dativ as Dat, Genitiv as Gen, Nominativ as Nom,
            Vokativ as Vok,
        },
        Numerus::{Plural as Pl, Singular as Sg},
        *,
    };

    macro_rules! test_deklination {
        ($nominativ:literal, $genitiv:literal, $genus:ident, $($kasus:ident $numerus:ident => $form:literal,)*) => {
            test_deklination!($nominativ, Some($genitiv), Some($genus), $($kasus $numerus => $form,)*);
        };
        ($nominativ:literal, $genitiv:literal, $($kasus:ident $numerus:ident => $form:literal,)*) => {
            test_deklination!($nominativ, Some($genitiv), None::<Genus>, $($kasus $numerus => $form,)*);
        };
        ($nominativ:literal, $($kasus:ident $numerus:ident => $form:literal,)*) => {
            test_deklination!($nominativ, None::<&str>, None::<Genus>, $($kasus $numerus => $form,)*);
        };
        ($nominativ:literal, $genitiv:expr, $genus:expr, $($kasus:ident $numerus:ident => $form:literal,)*) => {
            #[allow(unused)] let Some(nomen) = Nomen::parse(NomenDaten {
                nominativ_singular: $nominativ,
                genitiv_singular: $genitiv,
                genus: $genus
            }) else {
                panic!("failed to create nomen: {}, {:?}, {:?}", $nominativ, $genitiv, $genus);
            };
            $({
                let form = nomen.deklinieren($numerus, $kasus);
                assert_eq!(form, $form);
            })*
        };
    }

    #[test]
    fn test_o_dekl_mf() {
        test_deklination! {
            "servus",

            Nom Sg => "servus",
            Gen Sg => "servi",
            Dat Sg => "servo",
            Akk Sg => "servum",
            Abl Sg => "servo",
            Vok Sg => "serve",

            Nom Pl => "servi",
            Gen Pl => "servorum",
            Dat Pl => "servis",
            Akk Pl => "servos",
            Abl Pl => "servis",
            Vok Pl => "servi",
        };
    }

    #[test]
    fn test_o_dekl_n() {
        test_deklination! {
            "templum",

            Nom Sg => "templum",
            Gen Sg => "templi",
            Dat Sg => "templo",
            Akk Sg => "templum",
            Abl Sg => "templo",
            Vok Sg => "templum",

            Nom Pl => "templa",
            Gen Pl => "templorum",
            Dat Pl => "templis",
            Akk Pl => "templa",
            Abl Pl => "templis",
            Vok Pl => "templa",
        };
    }

    #[test]
    fn test_a_dekl() {
        test_deklination!{
            "amica",

            Nom Sg => "amica",
            Gen Sg => "amicae",
            Dat Sg => "amicae",
            Akk Sg => "amicam",
            Abl Sg => "amica",
            Vok Sg => "amica",

            Nom Pl => "amicae",
            Gen Pl => "amicarum",
            Dat Pl => "amicis",
            Akk Pl => "amicas",
            Abl Pl => "amicis",
            Vok Pl => "amicae",
        };
    }

    #[test]
    fn test_kons_dekl_mf() {
        test_deklination!{
            "senator", "senatoris", M,

            Nom Sg => "senator",
            Gen Sg => "senatoris",
            Dat Sg => "senatori",
            Akk Sg => "senatorem",
            Abl Sg => "senatore",
            Vok Sg => "senator",

            Nom Pl => "senatores",
            Gen Pl => "senatorum",
            Dat Pl => "senatoribus",
            Akk Pl => "senatores",
            Abl Pl => "senatoribus",
            Vok Pl => "senatores",
        };
    }

    #[test]
    fn test_kons_dekl_n() {
        test_deklination!{
            "onus", "oneris", N,

            Nom Sg => "onus",
            Gen Sg => "oneris",
            Dat Sg => "oneri",
            Akk Sg => "onus",
            Abl Sg => "onere",
            Vok Sg => "onus",

            Nom Pl => "onera",
            Gen Pl => "onerum",
            Dat Pl => "oneribus",
            Akk Pl => "onera",
            Abl Pl => "oneribus",
            Vok Pl => "onera",
        };
    }

    #[test]
    #[should_panic]
    fn test_kons_dekl_missing_genus() {
        test_deklination!("onus", "oneris",);
    }

    #[test]
    fn test_u_dekl() {
        test_deklination!{
            "senatus", "senatus",

            Nom Sg => "senatus",
            Gen Sg => "senatus",
            Dat Sg => "senatui",
            Akk Sg => "senatum",
            Abl Sg => "senatu",
            Vok Sg => "senatus",

            Nom Pl => "senatus",
            Gen Pl => "senatuum",
            Dat Pl => "senatibus",
            Akk Pl => "senatus",
            Abl Pl => "senatibus",
            Vok Pl => "senatus",
        };
    }

    #[test]
    fn test_e_dekl() {
        test_deklination!{
            "dies", "diei", M,

            Nom Sg => "dies",
            Gen Sg => "diei",
            Dat Sg => "diei",
            Akk Sg => "diem",
            Abl Sg => "die",
            Vok Sg => "dies",

            Nom Pl => "dies",
            Gen Pl => "dierum",
            Dat Pl => "diebus",
            Akk Pl => "dies",
            Abl Pl => "diebus",
            Vok Pl => "dies",
        };
    }
}
