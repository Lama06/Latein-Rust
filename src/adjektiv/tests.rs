use super::Adjektiv;
use crate::{
    adjektiv::WörterbuchEintrag,
    grammatik::{
        Genus::{Femininum as F, Maskulinum as M, Neutrum as N},
        Kasus::{
            Ablativ as Abl, Akkusativ as Akk, Dativ as Dat, Genitiv as Gen, Nominativ as Nom,
            Vokativ as Vok,
        },
        Numerus::{Plural as Pl, Singular as Sg},
        *,
    },
};

macro_rules! test_deklination {
    ($eintrag:expr; $($kasus:ident $numerus:ident $genus:ident => $form:literal),* $(,)?) => {
        let Some(adjektiv) = Adjektiv::parse(&$eintrag) else {
            panic!("failed to parse adjektiv: {:?}", $eintrag);
        };
        $({
            let form = adjektiv.deklinieren($genus, $numerus, $kasus);
            assert_eq!(form, $form);
        })*
    };
}

#[test]
fn test_a_o_dekl() {
    for eintrag in [
        WörterbuchEintrag::from_three("bonus", "bona", "bonum"),
        WörterbuchEintrag::from_three("bonus", "a", "um"),
        WörterbuchEintrag::from_one("bonus"),
    ] {
        test_deklination! {
            eintrag;

            Nom Sg M => "bonus",
            Gen Sg M => "boni",
            Dat Sg M => "bono",
            Akk Sg M => "bonum",
            Abl Sg M => "bono",
            Vok Sg M => "bone",
            Nom Pl M => "boni",
            Gen Pl M => "bonorum",
            Dat Pl M => "bonis",
            Akk Pl M => "bonos",
            Abl Pl M => "bonis",
            Vok Pl M => "boni",

            Nom Sg F => "bona",
            Gen Sg F => "bonae",
            Dat Sg F => "bonae",
            Akk Sg F => "bonam",
            Abl Sg F => "bona",
            Vok Sg F => "bona",
            Nom Pl F => "bonae",
            Gen Pl F => "bonarum",
            Dat Pl F => "bonis",
            Akk Pl F => "bonas",
            Abl Pl F => "bonis",
            Vok Pl F => "bonae",

            Nom Sg N => "bonum",
            Gen Sg N => "boni",
            Dat Sg N => "bono",
            Akk Sg N => "bonum",
            Abl Sg N => "bono",
            Vok Sg N => "bonum",
            Nom Pl N => "bona",
            Gen Pl N => "bonorum",
            Dat Pl N => "bonis",
            Akk Pl N => "bona",
            Abl Pl N => "bonis",
            Vok Pl N => "bona",
        };
    }
}

#[test]
fn test_kons_dekl_einendig() {
    test_deklination! {
        WörterbuchEintrag::from_two("vehemens", "vehementis");

        Nom Sg M => "vehemens",
        Gen Sg M => "vehementis",
        Dat Sg M => "vehementi",
        Akk Sg M => "vehementem",
        Abl Sg M => "vehementi",
        Vok Sg M => "vehemens",
        Nom Pl M => "vehementes",
        Gen Pl M => "vehementium",
        Dat Pl M => "vehementibus",
        Akk Pl M => "vehementes",
        Abl Pl M => "vehementibus",
        Vok Pl M => "vehementes",

        Nom Sg F => "vehemens",
        Gen Sg F => "vehementis",
        Dat Sg F => "vehementi",
        Akk Sg F => "vehementem",
        Abl Sg F => "vehementi",
        Vok Sg F => "vehemens",
        Nom Pl F => "vehementes",
        Gen Pl F => "vehementium",
        Dat Pl F => "vehementibus",
        Akk Pl F => "vehementes",
        Abl Pl F => "vehementibus",
        Vok Pl F => "vehementes",

        Nom Sg N => "vehemens",
        Gen Sg N => "vehementis",
        Dat Sg N => "vehementi",
        Akk Sg N => "vehemens",
        Abl Sg N => "vehementi",
        Vok Sg N => "vehemens",
        Nom Pl N => "vehementia",
        Gen Pl N => "vehementium",
        Dat Pl N => "vehementibus",
        Akk Pl N => "vehementia",
        Abl Pl N => "vehementibus",
        Vok Pl N => "vehementia",
    };
}
