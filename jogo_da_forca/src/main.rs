use rand::seq::SliceRandom;
use std::io;

fn generate_word() -> &'static str {
    let words = vec!["banana", "carro", "amor", "peixe"];
    let mut rng_words = rand::thread_rng();
    return words.choose(&mut rng_words).unwrap();
}

fn menu() {
    println!("*********************************");
    println!("*** BENVINDO AO JOGO DA FORCA ***");
    println!("*********************************");
}

fn generate_size(arg: Type) -> RetType {
    unimplemented!();
}

fn main() {
    menu();
    let word: &'static str = generate_word();
}
