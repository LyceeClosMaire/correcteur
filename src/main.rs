extern crate encoding_rs;
extern crate encoding_rs_io;

use std::cmp::min;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn lev(a : &str, b : &str) -> u64 {
    let n = a.chars().count();
    let m = b.chars().count();
    let mut distances : Vec<u64> = Vec::new();
    distances.resize( (n+1)*(m+1), 0);

    for i in 0..n+1 {
        for j in 0..m+1 {
            if i == 0 {
                distances[i*(m+1) + j] = j as u64;
            }
            else if j == 0 {
                distances[i*(m+1) + j] = i as u64;
            }
            else if a.chars().nth(i-1).unwrap() == b.chars().nth(j-1).unwrap() {
                distances[i*(m+1) + j] = distances[(i-1)*(m+1) + (j-1)]
            }
            else {
                distances[i*(m+1) + j] = 1 + min(distances[(i-1)*(m+1) + j], 
                                            min(distances[i*(m+1) + (j-1)], 
                                                distances[(i-1)*(m+1) + (j-1)]))
            }
        }
    }

    return distances[(n+1)*(m+1) - 1]
}

fn corrige (mut dico : Vec<String>, texte : String) {
    let mots : Vec<&str> = texte.split_whitespace().collect();
    for mot in mots.iter().filter(|m| m.len() >= 2) {
        let lower_mot = mot.to_lowercase();
        if !dico.contains(&lower_mot) {
            dico.sort_by_cached_key(|m| lev(&lower_mot, m));
            print!("Pour {}, suggestions : ", mot);
            for i in 0..15 {
                print!("{} ", dico[i]);
            }
            println!();
        }
    }
}

fn main() -> std::io::Result<()> {
    let decoder = DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1252)).build(File::open("liste_francais.txt").unwrap());
    let reader = BufReader::new(decoder);
    let dico : Vec<String> = reader.lines().into_iter().map(|m| m.unwrap()).collect();
    let mut texte = String::new();
    io::stdin().read_line(&mut texte).unwrap();
    corrige(dico, texte);
    
    return Ok(());
}
