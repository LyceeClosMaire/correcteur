#![feature(test)]

use levenshtein::lev;

extern crate test;

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    use encoding_rs::*;
    use encoding_rs_io::DecodeReaderBytesBuilder;
    use icu_normalizer::ComposingNormalizer;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[bench]
    fn bench_lev(b: &mut Bencher) {
        b.iter(|| lev("chiffre", "ami"));
    }

    #[bench]
    fn bench_small_dico(b: &mut Bencher) {
        let norm = ComposingNormalizer::new_nfc();
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open("liste_francais.txt").unwrap());
        let reader = BufReader::new(decoder);
        let dico: Vec<String> = reader
            .lines()
            .map(|m| norm.normalize(&m.unwrap()))
            .collect();

        b.iter(|| dico.clone().sort_by_cached_key(|m| lev("chifre", m)));
    }

    #[bench]
    fn bench_big_dico(b: &mut Bencher) {
        let norm = ComposingNormalizer::new_nfc();
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(UTF_8))
            .build(File::open("liste.de.mots.francais.frgut.txt").unwrap());
        let reader = BufReader::new(decoder);
        let dico: Vec<String> = reader
            .lines()
            .map(|m| norm.normalize(&m.unwrap()))
            .collect();

        b.iter(|| dico.clone().sort_by_cached_key(|m| lev("chifre", m)));
    }
}
