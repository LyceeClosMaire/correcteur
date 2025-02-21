# Distance de Levenshtein

## L'algorithme

Ce paquet Rust implémente la distance d'édition classique sous le nom `lev(a : &str, b : &str)` avec une programmation dynamique classique en bottom-up (résolution du simple au complexe).

Il offre également une fonction `corrige` prenant une liste de mots (un dictionnaire) et un texte à corriger et proposant les 15 plus proches mots du dictionnaire pour chaque mot présent dans le texte mais pas dans le dictionnaire.

Deux dictionnaires sont présents dans le dossier, un simple mais incomplet (~22&nbsp;000 mots en cp1252) et un bien plus complet (~330&nbsp;000 mots en UTF-8, FND).

## Test de performance

`cargo bench` réalise trois tests : un calcul de distance d'édition simple, un tri par distance au mot "chifre" pour chacun des deux dictionnaires.

## Résultats

Sur une même machine (M4 Mac Mini), les résultats sont les suivantes :

```
test benches::bench_big_dico   ... bench:  73,212,970.80 ns/iter (+/- 2,616,989.51)
test benches::bench_lev        ... bench:          70.39 ns/iter (+/- 2.18)
test benches::bench_small_dico ... bench:   4,365,600.00 ns/iter (+/- 171,811.44)
```

C'est-à-dire : de l'ordre de 100ns ( $10^{-7}$ s) pour un calcul de distance ; 4ms pour le petit dictionnaire et 70ms pour le grand.

Sur Capytale, exécuté par Basthon (donc via JavaScript), un code utilisant le même algorithme sous Python réalise le même tri avec le petit dictionnaire en 860ms, soit 200 fois plus lentement.

Sur Python 3.12.7, le script python_lev.py met 160ms pour s'exécuter, soit 5 fois plus vite que sur Capytale mais toujours 40 fois plus lentement que le code Rust.
