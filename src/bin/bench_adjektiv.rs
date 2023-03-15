use std::{time::Instant, io::stdin};

use latein_rs::{adjektiv::{Adjektiv, WörterbuchEintrag}, grammatik::{Genus, Numerus, Kasus}};

const ITERATIONS: u32 = 1_000_000;

fn main() {
    let mut result = Vec::with_capacity(Genus::ALLE.len() * Numerus::ALLE.len() * Kasus::ALLE.len() * ITERATIONS as usize);

    let start = Instant::now();
    for _ in 1..=ITERATIONS {
        let adjektiv = Adjektiv::parse(&WörterbuchEintrag::from_three("acer", "acris", "acre")).unwrap();
        for genus in Genus::ALLE {
            for numerus in Numerus::ALLE {
                for kasus in Kasus::ALLE {
                    result.push(adjektiv.deklinieren(genus, numerus, kasus));
                }
            }
        }
    }

    println!("Dieses Adjektiv {}x mal durchzudeklinieren hat {}s gedauert ({} Formen)", ITERATIONS, start.elapsed().as_secs_f64(), result.len());
    stdin().read_line(&mut String::new()).unwrap();
}