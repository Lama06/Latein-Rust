use super::{Nomen, WörterbuchEintrag};
use crate::grammatik::{
    Genus::{Maskulinum as M, Neutrum as N},
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
            #[allow(unused)] let Some(nomen) = Nomen::parse(&WörterbuchEintrag {
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
