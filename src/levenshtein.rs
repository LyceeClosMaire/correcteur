use std::cmp::min;
use regex::Regex;
use ndarray::prelude::*;

pub fn lev(a : &str, b : &str) -> u64 {
    let av : Vec<_> = a.chars().collect();
    let bv : Vec<_> = b.chars().collect();
    let n = av.len();
    let m = bv.len();
    let mut distances : Array<u64, _> = Array::zeros((n+1,m+1));

    for i in 0..n+1 {
        for j in 0..m+1 {
            if i == 0 {
                distances[(i,j)] = j as u64;
            }
            else if j == 0 {
                distances[(i,j)] = i as u64;
            }
            else if av[i-1] == bv[j-1] {
                distances[(i,j)] = distances[(i-1,j-1)]
            }
            else {
                distances[(i,j)] = 1 + min(distances[(i-1,j)], 
                                            min(distances[(i,j-1)], 
                                                distances[(i-1,j-1)]))
            }
        }
    }

    return distances[(n,m)]
}

pub fn corrige (mut dico : Vec<String>, texte : String) {
    let regex_mot = Regex::new(r"\w+['’]?").unwrap();
    let mots : Vec<&str> = 
        regex_mot.find_iter(&texte)
            .map(|m| m.as_str())
            .filter(|w| {let last = w.chars().last(); last != Some('\'') && last != Some('’')})
            .collect();
    for mot in mots.iter() {
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