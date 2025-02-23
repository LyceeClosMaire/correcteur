import time

# chargeons la liste des mots français
mots = []
with open("liste_francais.txt", "r", encoding="cp1252") as fichier_dico:
    for mot in fichier_dico:
        # suppression du caractère de retour à la ligne
        mots.append(mot[:-1])


def lev(a: str, b: str) -> int:
    """Renvoie la distance de Levenshtein entre a et b

    >>> lev("château", "chapeau")
    2
    """
    n, m = len(a), len(b)

    # ne pas utiliser [[0] * (m+1)] * (n+1)
    D = [[0 for _ in range(m + 1)] for _ in range(n + 1)]

    for i in range(n + 1):
        for j in range(m + 1):
            if j == 0:
                D[i][j] = i
            elif i == 0:
                D[i][j] = j
            elif a[i - 1] == b[j - 1]:
                D[i][j] = D[i - 1][j - 1]
            else:
                D[i][j] = 1 + min(D[i - 1][j], D[i][j - 1], D[i - 1][j - 1])

    return D[n][m]


assert lev("château", "chapeau") == 2
assert lev("chapeau", "chance") == 4

N = 22_735
a = "chifre"

start = time.perf_counter()

for i in range(N):
    b = mots[i]
    d = lev(a, b)

end = time.perf_counter()
print(f"Temps écoulé : {end - start}s")
