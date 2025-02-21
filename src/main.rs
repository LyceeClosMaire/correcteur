use std::io::{self, BufRead, BufReader};
use std::fs::File;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod levenshtein;

fn main() -> std::io::Result<()> {
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(File::open("liste_francais.txt")
        .unwrap());
    let reader = BufReader::new(decoder);
    let dico : Vec<String> = reader.lines().map(|m| m.unwrap()).collect();
    let mut texte = String::new();

    println!("Entrez votre texte à corriger à la ligne suivante :");
    io::stdin().read_line(&mut texte).unwrap();
    
    levenshtein::corrige(dico, texte);
    
    return Ok(());
}
