use std::{io::stdin, time::Instant};

use latein_rs::{
    grammatik::{Genus, Kasus, Numerus},
    nomen::{Nomen, WörterbuchEintrag},
};

const ITERATIONS: u32 = 1_000_000;

fn main() {
    let mut result =
        Vec::with_capacity(ITERATIONS as usize * Numerus::ALLE.len() * Kasus::ALLE.len());

    let start = Instant::now();

    for i in 1..=ITERATIONS {
        let Some(nomen) = Nomen::parse(&WörterbuchEintrag {
            nominativ: "senator",
            genitiv: Some("senatoris"),
            genus: Some(Genus::Maskulinum),
        }) else {
            panic!("invalid arguments");
        };
        for numerus in Numerus::ALLE {
            for kasus in Kasus::ALLE {
                let form = nomen.deklinieren(numerus, kasus);
                result.push(form);
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
