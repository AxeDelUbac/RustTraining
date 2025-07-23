use std::io;

mod cesar;

fn main() {
    let offset = 5;

    println!("Entrez un mot :");
    let mut input_word = String::new();
    io::stdin().read_line(&mut input_word).expect("Erreur de lecture");
    println!("Vous avez entré : {}", input_word.trim());

    println!("voulez vous encoder ou décoder : 1 pour encoder, 2 pour décoder");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Erreur de lecture");

    let output_word = match choice.trim() {
        "1" => cesar::encrypt(input_word.trim(), offset),
        "2" => cesar::decrypt(input_word.trim(), offset),
        _ => String::from("Choix invalide"),
    };

    println!("Original word: {}, outputword: {}", input_word, output_word);
}