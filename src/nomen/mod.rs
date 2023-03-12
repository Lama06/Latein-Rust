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

fn test_form(form: &str, stamm: &str, endung: &str) -> bool {
    form.starts_with(stamm) && form.ends_with(endung) && form.len() == stamm.len() + endung.len()
}

trait Deklination {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String>;
}

trait ParsableDeklination<'a>: Deklination + Sized {
    const DEFAULT_GENUS: Option<Genus>;
    const ALLOWS_FEMININUM: bool;
    const ALLOWS_MASKULINUM: bool;
    const ALLOWS_NEUTRUM: bool;

    fn parse_wörterbuch_formen(nominativ: &'a str, genitiv: Option<&'a str>) -> Option<Self>;

    fn parse_wörterbuch_eintrag(eintrag: &WörterbuchEintrag<'a>) -> Option<(Genus, Self)> {
        let genus = match eintrag.genus {
            None => match Self::DEFAULT_GENUS {
                Some(default) => default,
                None => return None,
            },
            Some(Genus::Maskulinum) if !Self::ALLOWS_MASKULINUM => return None,
            Some(Genus::Femininum) if !Self::ALLOWS_FEMININUM => return None,
            Some(Genus::Neutrum) if !Self::ALLOWS_NEUTRUM => return None,
            Some(genus) => genus,
        };

        let deklination = match Self::parse_wörterbuch_formen(eintrag.nominativ, eintrag.genitiv) {
            Some(deklination) => deklination,
            None => return None,
        };

        Some((genus, deklination))
    }
}

trait StammDeklination<'a>: Deklination + Sized {
    const DEFAULT_GENUS: Option<Genus>;
    const ALLOWS_FEMININUM: bool;
    const ALLOWS_MASKULINUM: bool;
    const ALLOWS_NEUTRUM: bool;

    const PLURAL: bool = false;
    const REQUIRE_GENITIVE: bool = false;

    fn from_stamm(stamm: &'a str) -> Self;

    fn get_stamm(&self) -> &str;

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str>;

    fn get_endung_instance(&self, numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        None
    }
}

impl<'a, T: StammDeklination<'a>> Deklination for T {
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        if Self::PLURAL && matches!(numerus, Numerus::Singular) {
            return None;
        }

        let mut result = String::new();
        result.push_str(self.get_stamm());
        result.push_str(match Self::get_endung(numerus, kasus) {
            Some(endung) => endung,
            None => match self.get_endung_instance(numerus, kasus) {
                Some(endung) => endung,
                None => return None,
            },
        });
        Some(result)
    }
}

impl<'a, T: StammDeklination<'a>> ParsableDeklination<'a> for T {
    const DEFAULT_GENUS: Option<Genus> = Self::DEFAULT_GENUS;
    const ALLOWS_FEMININUM: bool = Self::ALLOWS_FEMININUM;
    const ALLOWS_MASKULINUM: bool = Self::ALLOWS_MASKULINUM;
    const ALLOWS_NEUTRUM: bool = Self::ALLOWS_NEUTRUM;

    fn parse_wörterbuch_formen(nominativ: &'a str, genitiv: Option<&'a str>) -> Option<Self> {
        let numerus: Numerus = if Self::PLURAL {
            Numerus::Plural
        } else {
            Numerus::Singular
        };
        let nominativ_endung = match Self::get_endung(numerus, Kasus::Nominativ) {
            Some(endung) => endung,
            None => return None,
        };
        let genitiv_endung = match Self::get_endung(numerus, Kasus::Genitiv) {
            Some(endung) => endung,
            None => return None,
        };

        let stamm = if nominativ.ends_with(nominativ_endung) {
            &nominativ[..nominativ.len() - nominativ_endung.len()]
        } else {
            return None;
        };

        if let Some(genitiv) = genitiv {
            if !test_form(genitiv, stamm, genitiv_endung) {
                return None;
            }
        } else if Self::REQUIRE_GENITIVE {
            return None;
        }

        Some(T::from_stamm(stamm))
    }
}

struct PluralDeklination<T>(T)
where
    T: Deklination;

impl<'a, T> StammDeklination<'a> for PluralDeklination<T>
where
    T: Deklination + StammDeklination<'a>,
{
    const DEFAULT_GENUS: Option<Genus> = T::DEFAULT_GENUS;
    const ALLOWS_MASKULINUM: bool = T::ALLOWS_MASKULINUM;
    const ALLOWS_FEMININUM: bool = T::ALLOWS_FEMININUM;
    const ALLOWS_NEUTRUM: bool = T::ALLOWS_NEUTRUM;

    const PLURAL: bool = true;
    const REQUIRE_GENITIVE: bool = true;

    fn from_stamm(stamm: &'a str) -> Self {
        PluralDeklination(T::from_stamm(stamm))
    }

    fn get_stamm(&self) -> &str {
        self.0.get_stamm()
    }

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        T::get_endung(numerus, kasus)
    }

    fn get_endung_instance(&self, numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        self.0.get_endung_instance(numerus, kasus)
    }
}

#[derive(Clone, Copy)]
pub struct WörterbuchEintrag<'a> {
    pub nominativ: &'a str,
    pub genitiv: Option<&'a str>,
    pub genus: Option<Genus>,
}

impl<'a> WörterbuchEintrag<'a> {
    fn parse(&self) -> Option<(Genus, Box<dyn Deklination + 'a>)> {
        macro_rules! parse_deklination {
            ($deklination:path) => {
                parse_deklination!(@PARSE $deklination);
                parse_deklination!(@PARSE PluralDeklination::<$deklination>);
            };
            (@PARSE $deklination:path) => {
                if let Some((genus, deklination)) = <$deklination>::parse_wörterbuch_eintrag(self) {
                    return Some((genus, Box::new(deklination)))
                }
            };
        }

        parse_deklination!(KonsonantischeDeklinationMaskulinumFemininum);
        parse_deklination!(KonsonantischeDeklinationNeutrum);
        parse_deklination!(ODeklinationMaskulinumFemininum);
        parse_deklination!(ODeklinationNeutrum);
        parse_deklination!(ADeklination);
        parse_deklination!(EDeklination);
        parse_deklination!(UDeklination);

        None
    }
}

pub struct Nomen {
    genus: Genus,
    formen: HashMap<(Numerus, Kasus), String>,
}

impl Nomen {
    pub fn parse(daten: WörterbuchEintrag) -> Option<Self> {
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
                        formen.insert(
                            (numerus, kasus),
                            match deklination.deklinieren(numerus, kasus) {
                                Some(form) => form,
                                None => continue,
                            },
                        );
                    }
                }

                formen
            },
        })
    }

    pub fn get_genus(&self) -> Genus {
        self.genus
    }

    pub fn deklinieren<'a>(&'a self, numerus: Numerus, kasus: Kasus) -> Option<&'a str> {
        match self.formen.get(&(numerus, kasus)) {
            Some(form) => Some(form),
            None => None,
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
        ($nominativ:literal, $genitiv:literal, $genus:ident, $($kasus:ident $numerus:ident => $form:literal),* $(,)?) => {
            test_deklination!(@PRIVATE; $nominativ, Some($genitiv), Some($genus), $($kasus $numerus => $form,)*);
        };
        ($nominativ:literal, $genitiv:literal, $($kasus:ident $numerus:ident => $form:literal),* $(,)?) => {
            test_deklination!(@PRIVATE; $nominativ, Some($genitiv), None::<Genus>, $($kasus $numerus => $form,)*);
        };
        ($nominativ:literal, $($kasus:ident $numerus:ident => $form:literal),* $(,)?) => {
            test_deklination!(@PRIVATE; $nominativ, None::<&str>, None::<Genus>, $($kasus $numerus => $form,)*);
        };
        (@PRIVATE; $nominativ:literal, $genitiv:expr, $genus:expr, $($kasus:ident $numerus:ident => $form:literal,)*) => {
            #[allow(unused)] let Some(nomen) = Nomen::parse(WörterbuchEintrag {
                nominativ: $nominativ,
                genitiv: $genitiv,
                genus: $genus
            }) else {
                panic!("failed to create nomen: {}, {:?}, {:?}", $nominativ, $genitiv, $genus);
            };
            $({
                let form = nomen.deklinieren($numerus, $kasus);
                assert_eq!(form.unwrap(), $form);
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
    fn test_o_dekl_mf_pl() {
        test_deklination! {
            "servi", "servorum",

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
    fn test_o_dekl_n_pl() {
        test_deklination! {
            "templa", "templorum",

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
        test_deklination! {
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
    fn test_a_dekl_pl() {
        test_deklination! {
            "amica", "amicae",

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
        test_deklination! {
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
    fn test_kons_dekl_mf_pl() {
        test_deklination! {
            "senatores", "senatorum", M,

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
        test_deklination! {
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
    fn test_kons_dekl_n_pl() {
        test_deklination! {
            "onera", "onerum", N,

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
        test_deklination! {
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
    fn test_u_dekl_pl() {
        test_deklination! {
            "senatus", "senatuum", M,

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
        test_deklination! {
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

    #[test]
    fn test_e_dekl_pl() {
        test_deklination! {
            "dies", "dierum", M,

            Nom Pl => "dies",
            Gen Pl => "dierum",
            Dat Pl => "diebus",
            Akk Pl => "dies",
            Abl Pl => "diebus",
            Vok Pl => "dies",
        };
    }
}
