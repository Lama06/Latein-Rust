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
        Steigerung,
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
    (adjektiv => $adjektiv:expr; $($kasus:ident $numerus:ident $genus:ident => $form:literal),* $(,)?) => {
        let adjektiv = $adjektiv;
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
fn test_a_o_dekl_er() {
    test_deklination! {
        WörterbuchEintrag::from_three("pulcher", "pulchra", "pulchrum");

        Nom Sg M => "pulcher",
        Gen Sg M => "pulchri",
        Dat Sg M => "pulchro",
        Akk Sg M => "pulchrum",
        Abl Sg M => "pulchro",
        Vok Sg M => "pulchre",
        Nom Pl M => "pulchri",
        Gen Pl M => "pulchrorum",
        Dat Pl M => "pulchris",
        Akk Pl M => "pulchros",
        Abl Pl M => "pulchris",
        Vok Pl M => "pulchri",

        Nom Sg F => "pulchra",
        Gen Sg F => "pulchrae",
        Dat Sg F => "pulchrae",
        Akk Sg F => "pulchram",
        Abl Sg F => "pulchra",
        Vok Sg F => "pulchra",
        Nom Pl F => "pulchrae",
        Gen Pl F => "pulchrarum",
        Dat Pl F => "pulchris",
        Akk Pl F => "pulchras",
        Abl Pl F => "pulchris",
        Vok Pl F => "pulchrae",

        Nom Sg N => "pulchrum",
        Gen Sg N => "pulchri",
        Dat Sg N => "pulchro",
        Akk Sg N => "pulchrum",
        Abl Sg N => "pulchro",
        Vok Sg N => "pulchrum",
        Nom Pl N => "pulchra",
        Gen Pl N => "pulchrorum",
        Dat Pl N => "pulchris",
        Akk Pl N => "pulchra",
        Abl Pl N => "pulchris",
        Vok Pl N => "pulchra",
    };
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

#[test]
fn test_kons_dekl_zweiendig() {
    for eintrag in [
        WörterbuchEintrag::from_two("fortis", "forte"),
        WörterbuchEintrag::from_two("fortis", "e"),
    ] {
        test_deklination! {
            eintrag;

            Nom Sg M => "fortis",
            Gen Sg M => "fortis",
            Dat Sg M => "forti",
            Akk Sg M => "fortem",
            Abl Sg M => "forti",
            Vok Sg M => "fortis",
            Nom Pl M => "fortes",
            Gen Pl M => "fortium",
            Dat Pl M => "fortibus",
            Akk Pl M => "fortes",
            Abl Pl M => "fortibus",
            Vok Pl M => "fortes",

            Nom Sg F => "fortis",
            Gen Sg F => "fortis",
            Dat Sg F => "forti",
            Akk Sg F => "fortem",
            Abl Sg F => "forti",
            Vok Sg F => "fortis",
            Nom Pl F => "fortes",
            Gen Pl F => "fortium",
            Dat Pl F => "fortibus",
            Akk Pl F => "fortes",
            Abl Pl F => "fortibus",
            Vok Pl F => "fortes",

            Nom Sg N => "forte",
            Gen Sg N => "fortis",
            Dat Sg N => "forti",
            Akk Sg N => "forte",
            Abl Sg N => "forti",
            Vok Sg N => "forte",
            Nom Pl N => "fortia",
            Gen Pl N => "fortium",
            Dat Pl N => "fortibus",
            Akk Pl N => "fortia",
            Abl Pl N => "fortibus",
            Vok Pl N => "fortia",
        };
    }
}

#[test]
fn test_kons_dekl_dreieindig() {
    test_deklination! {
        WörterbuchEintrag::from_three("acer", "acris", "acre");

        Nom Sg M => "acer",
        Gen Sg M => "acris",
        Dat Sg M => "acri",
        Akk Sg M => "acrem",
        Abl Sg M => "acri",
        Vok Sg M => "acer",
        Nom Pl M => "acres",
        Gen Pl M => "acrium",
        Dat Pl M => "acribus",
        Akk Pl M => "acres",
        Abl Pl M => "acribus",
        Vok Pl M => "acres",

        Nom Sg F => "acris",
        Gen Sg F => "acris",
        Dat Sg F => "acri",
        Akk Sg F => "acrem",
        Abl Sg F => "acri",
        Vok Sg F => "acris",
        Nom Pl F => "acres",
        Gen Pl F => "acrium",
        Dat Pl F => "acribus",
        Akk Pl F => "acres",
        Abl Pl F => "acribus",
        Vok Pl F => "acres",

        Nom Sg N => "acre",
        Gen Sg N => "acris",
        Dat Sg N => "acri",
        Akk Sg N => "acre",
        Abl Sg N => "acri",
        Vok Sg N => "acre",
        Nom Pl N => "acria",
        Gen Pl N => "acrium",
        Dat Pl N => "acribus",
        Akk Pl N => "acria",
        Abl Pl N => "acribus",
        Vok Pl N => "acria",
    };
}

#[test]
fn test_superlativ_ao() {
    test_deklination! {
        adjektiv => Adjektiv::parse(&WörterbuchEintrag::from_one("longus")).unwrap()
            .steigern(Steigerung::Superlativ).unwrap();

        Nom Sg M => "longissimus",
        Gen Sg M => "longissimi",
        Dat Sg M => "longissimo",
        Akk Sg M => "longissimum",
        Abl Sg M => "longissimo",
        Vok Sg M => "longissime",
        Nom Pl M => "longissimi",
        Gen Pl M => "longissimorum",
        Dat Pl M => "longissimis",
        Akk Pl M => "longissimos",
        Abl Pl M => "longissimis",
        Vok Pl M => "longissimi",

        Nom Sg F => "longissima",
        Gen Sg F => "longissimae",
        Dat Sg F => "longissimae",
        Akk Sg F => "longissimam",
        Abl Sg F => "longissima",
        Vok Sg F => "longissima",
        Nom Pl F => "longissimae",
        Gen Pl F => "longissimarum",
        Dat Pl F => "longissimis",
        Akk Pl F => "longissimas",
        Abl Pl F => "longissimis",
        Vok Pl F => "longissimae",

        Nom Sg N => "longissimum",
        Gen Sg N => "longissimi",
        Dat Sg N => "longissimo",
        Akk Sg N => "longissimum",
        Abl Sg N => "longissimo",
        Vok Sg N => "longissimum",
        Nom Pl N => "longissima",
        Gen Pl N => "longissimorum",
        Dat Pl N => "longissimis",
        Akk Pl N => "longissima",
        Abl Pl N => "longissimis",
        Vok Pl N => "longissima",
    };
}

#[test]
fn test_adverb() {
    assert_eq!(
        Adjektiv::parse(&WörterbuchEintrag::from_one("bonus"))
            .unwrap()
            .adverb(),
        "bone"
    );
    assert_eq!(
        Adjektiv::parse(&WörterbuchEintrag::from_three(
            "pulcher", "pulchra", "pulchrum"
        ))
        .unwrap()
        .adverb(),
        "pulchre"
    );
    assert_eq!(
        Adjektiv::parse(&WörterbuchEintrag::from_two("fortis", "e"))
            .unwrap()
            .adverb(),
        "fortiter"
    );
    assert_eq!(
        Adjektiv::parse(&WörterbuchEintrag::from_three("acer", "acris", "acre"))
            .unwrap()
            .adverb(),
        "acriter"
    );
    assert_eq!(
        Adjektiv::parse(&WörterbuchEintrag::from_two("vehemens", "vehementis"))
            .unwrap()
            .adverb(),
        "vehementer"
    );
}
