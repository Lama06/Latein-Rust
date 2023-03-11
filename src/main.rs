use std::io::stdin;

use grammatik::{Genus, Numerus, Kasus};
use nomen::{Nomen, NomenDaten};

mod grammatik;
mod nomen;

fn main() {
    let nominativ_singular = {
        println!("Nominativ Singular:");
        let mut input = String::new();
        stdin().read_line(&mut input);
        println!();
        String::from(input.trim())
    };
    let genitiv_singular = {
        println!("Genitiv Singular:");
        let mut input = String::new();
        stdin().read_line(&mut input);
        println!();
        if input.trim().is_empty() {
            None
        } else {
            Some(String::from(input.trim()))
        }
    };
    let genus = 'genus: {
        println!("Geschlecht:");
        let mut input = String::new();
        stdin().read_line(&mut input);
        println!();
        if input.trim().len() == 0 {
            break 'genus None
        }
        if input.trim().len() != 1 {
            panic!("only one letter allowed");
        }
        let letter = input.chars().nth(0).unwrap();
        for genus in Genus::ALLE {
            if letter == genus.get_letter() {
                break 'genus Some(genus)
            }
        }
        panic!("invalid genus");
    };

    let daten = NomenDaten {
        nominativ_singular: &nominativ_singular,
        genitiv_singular: match genitiv_singular {
            Some(ref genitiv_singular) => Some(genitiv_singular),
            None => None,
        },
        genus,
    };
    let nomen = match Nomen::parse(daten) {
        Some(nomen) => nomen,
        None => panic!("invalid arguments"),
    };

    println!("Formen:");
    for numerus in Numerus::ALLE {
        for kasus in Kasus::ALLE {
            let form = nomen.deklinieren(numerus, kasus);
            println!("{:?} {:?} => {}", kasus, numerus, form);
        }
    }
    
    stdin().read_line(&mut String::new()).unwrap();
}
