use std::io::stdin;

use latein_rs::{
    grammatik::{Genus, Kasus, Numerus},
    nomen::{Nomen, WörterbuchEintrag},
};

fn main() {
    'main: loop {
        let nominativ_singular = {
            println!("Nominativ:");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            println!();
            String::from(input.trim())
        };
        let genitiv_singular = {
            println!("Genitiv:");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
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
            stdin().read_line(&mut input).unwrap();
            println!();
            if input.trim().len() == 0 {
                break 'genus None;
            }
            if input.trim().len() != 1 {
                println!("only one letter allowed");
                continue 'main;
            }
            let letter = input.trim().chars().nth(0).unwrap();
            for genus in Genus::ALLE {
                if letter == genus.get_letter() {
                    break 'genus Some(genus);
                }
            }
            println!("invalid genus");
            continue 'main;
        };

        let eintrag = WörterbuchEintrag {
            nominativ: &nominativ_singular,
            genitiv: match genitiv_singular {
                Some(ref genitiv_singular) => Some(genitiv_singular),
                None => None,
            },
            genus,
        };
        let Some(nomen) = Nomen::parse(&eintrag) else {
            println!("invalid arguments");
            continue;
        };

        println!("Geschlecht: {:?}", nomen.get_genus());
        println!();
        for numerus in Numerus::ALLE {
            for kasus in Kasus::ALLE {
                let form = nomen.deklinieren(numerus, kasus);
                println!(
                    "{:?} {:?} => {}",
                    kasus,
                    numerus,
                    match form {
                        Some(form) => form,
                        None => {
                            println!("-");
                            continue;
                        }
                    }
                );
            }
            println!();
        }
        println!();
    }
}
