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

#[cfg(test)]
mod tests;

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

trait StammDeklination<'a>: Sized {
    const DEFAULT_GENUS: Option<Genus>;
    const ALLOWS_FEMININUM: bool;
    const ALLOWS_MASKULINUM: bool;
    const ALLOWS_NEUTRUM: bool;

    const REQUIRE_GENITIVE_SINGULAR: bool = false;
    const REQUIRE_GENITIVE_PLURAL: bool = true;

    fn new(stamm: &'a str, plural: bool) -> Self;

    fn get_stamm(&self) -> &str;

    fn is_plural(&self) -> bool;

    fn get_endung(numerus: Numerus, kasus: Kasus) -> Option<&'static str>;

    fn get_endung_instance(&self, numerus: Numerus, kasus: Kasus) -> Option<&'static str> {
        None
    }
}

impl<'a, T> Deklination for T
where
    T: StammDeklination<'a>,
{
    fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        if self.is_plural() && matches!(numerus, Numerus::Singular) {
            return None;
        }

        let stamm = self.get_stamm();
        let endung = match Self::get_endung(numerus, kasus) {
            Some(endung) => endung,
            None => match self.get_endung_instance(numerus, kasus) {
                Some(endung) => endung,
                None => return None,
            },
        };

        let mut result = String::with_capacity(stamm.len() + endung.len());
        result.push_str(stamm);
        result.push_str(endung);
        Some(result)
    }
}

impl<'a, T> ParsableDeklination<'a> for T
where
    T: StammDeklination<'a>,
{
    const DEFAULT_GENUS: Option<Genus> = Self::DEFAULT_GENUS;
    const ALLOWS_FEMININUM: bool = Self::ALLOWS_FEMININUM;
    const ALLOWS_MASKULINUM: bool = Self::ALLOWS_MASKULINUM;
    const ALLOWS_NEUTRUM: bool = Self::ALLOWS_NEUTRUM;

    fn parse_wörterbuch_formen(nominativ: &'a str, genitiv: Option<&'a str>) -> Option<Self> {
        for numerus in Numerus::ALLE {
            let nominativ_endung = match T::get_endung(numerus, Kasus::Nominativ) {
                Some(endung) => endung,
                None => continue,
            };
            let genitiv_endung = match T::get_endung(numerus, Kasus::Genitiv) {
                Some(endung) => endung,
                None => continue,
            };

            let stamm = if nominativ.ends_with(nominativ_endung) {
                &nominativ[..nominativ.len() - nominativ_endung.len()]
            } else {
                continue;
            };

            if let Some(genitiv) = genitiv {
                if !test_form(genitiv, stamm, genitiv_endung) {
                    continue;
                }
            } else if match numerus {
                Numerus::Singular => T::REQUIRE_GENITIVE_SINGULAR,
                Numerus::Plural => T::REQUIRE_GENITIVE_PLURAL,
            } {
                continue;
            }

            return Some(T::new(stamm, matches!(numerus, Numerus::Plural)));
        }

        None
    }
}

#[derive(Clone, Copy)]
pub struct WörterbuchEintrag<'a> {
    pub nominativ: &'a str,
    pub genitiv: Option<&'a str>,
    pub genus: Option<Genus>,
}

impl<'a> WörterbuchEintrag<'a> {
    fn parse_deklination<T>(&self) -> Option<(Genus, Box<dyn Deklination + 'a>)>
    where
        T: ParsableDeklination<'a> + 'a,
    {
        match T::parse_wörterbuch_eintrag(self) {
            Some((genus, deklination)) => Some((genus, Box::new(deklination))),
            None => None,
        }
    }

    fn parse(&self) -> Option<(Genus, Box<dyn Deklination + 'a>)> {
        if let result @ Some(_) =
            self.parse_deklination::<KonsonantischeDeklinationMaskulinumFemininum>()
        {
            result
        } else if let result @ Some(_) =
            self.parse_deklination::<KonsonantischeDeklinationNeutrum>()
        {
            result
        } else if let result @ Some(_) = self.parse_deklination::<ODeklinationMaskulinumFemininum>()
        {
            result
        } else if let result @ Some(_) = self.parse_deklination::<ODeklinationNeutrum>() {
            result
        } else if let result @ Some(_) = self.parse_deklination::<ADeklination>() {
            result
        } else if let result @ Some(_) = self.parse_deklination::<EDeklination>() {
            result
        } else if let result @ Some(_) = self.parse_deklination::<UDeklination>() {
            result
        } else {
            None
        }
    }
}

pub struct Nomen<'a> {
    genus: Genus,
    deklination: Box<dyn Deklination + 'a>,
}

impl<'a> Nomen<'a> {
    pub fn parse(eintrag: WörterbuchEintrag<'a>) -> Option<Self> {
        let (genus, deklination) = match eintrag.parse() {
            Some(result) => result,
            None => return None,
        };

        Some(Self { genus, deklination })
    }

    pub fn get_genus(&self) -> Genus {
        self.genus
    }

    pub fn deklinieren(&self, numerus: Numerus, kasus: Kasus) -> Option<String> {
        self.deklination.deklinieren(numerus, kasus)
    }
}
