use std::{collections::HashMap, io::stdin, time::Instant};

use latein_rs::{
    grammatik::{Genus, Kasus, Numerus},
    nomen::{Nomen, WörterbuchEintrag},
};

const ITERATIONS: u32 = 1_000_000;

fn main() {
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
            panic!("only one letter allowed");
        }
        let letter = input.trim().chars().nth(0).unwrap();
        for genus in Genus::ALLE {
            if letter == genus.get_letter() {
                break 'genus Some(genus);
            }
        }
        panic!("invalid genus");
    };

    let mut result =
        HashMap::with_capacity(ITERATIONS as usize * Numerus::ALLE.len() * Kasus::ALLE.len());

    let start = Instant::now();

    for i in 1..=ITERATIONS {
        let Some(nomen) = Nomen::parse(&WörterbuchEintrag {
            nominativ: &nominativ_singular,
            genitiv: match genitiv_singular {
                Some(ref genitiv) => Some(genitiv),
                None => None,
            },
            genus,
        }) else {
            panic!("invalid arguments");
        };
        for numerus in Numerus::ALLE {
            for kasus in Kasus::ALLE {
                let form = nomen.deklinieren(numerus, kasus);
                result.insert((i, numerus, kasus), form);
            }
        }
    }

    println!(
        "Dieses Nomen {}x durchzudeklinieren, hat {}s gedauert ({} Formen).",
        ITERATIONS,
        start.elapsed().as_secs_f64(),
        result.len()
    );
    stdin().read_line(&mut String::new()).unwrap();
}
