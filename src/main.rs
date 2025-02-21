use std::io::{self, BufRead, BufReader};
use std::fs::File;
use encoding_rs::*;
use encoding_rs_io::DecodeReaderBytesBuilder;
use icu_normalizer::ComposingNormalizer;

use levenshtein;

fn main() -> std::io::Result<()> {
    let norm = ComposingNormalizer::new_nfc();
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8))
        .build(File::open("liste.de.mots.francais.frgut.txt")
        .unwrap());
    let reader = BufReader::new(decoder);
    let dico : Vec<String> = reader.lines().map(|m| norm.normalize(&m.unwrap())).collect();
    let mut texte = String::new();

    println!("Entrez votre texte à corriger à la ligne suivante :");
    io::stdin().read_line(&mut texte).unwrap();

    levenshtein::corrige(dico, texte);
    
    return Ok(());
}
