Dieses Projekt implementiert eine leicht abgeänderte Version der WHILE-Sprache.

# Nutzung

Der Parser und Interpreter sind mit Rust als Cargo-Projekt umgesetzt. Sofern
beides korrekt installiert ist, kann mit `cargo r <datei>` bzw. `cargo r
--release <datei>` eine Datei, die ein WHILE-Programm enthält, geparst und
interpretiert werden. Die Ausgabe besteht aus dem AST gefolgt von der Ausgabe
des Programms. Das Beispielprogramm in `sum.while` etwa kann geparst und
ausgeführt werden mit `cargo r sum.while`.

# Umsetzung

Als Parsergenerator wurde [`lalrpop`](https://github.com/lalrpop/lalrpop)
benutzt, welches `LR(1)`-Parser generiert. Entsprechend ist die Grammatik und
der generierte Parser `LR(1)`.Der einzige Input für den Generator ist die
Grammatikdatei `src/while_grammar.lalrpop` und die AST-Definitionen in
`src/ast.rs`. 

Das Dangling-Else-Problem stellte sich wegen der obligatorischen geschweiften
Klammern um den Körper der IF-Ausdrücke nicht. Nur die Assoziativität der
Sequenzen mit Semikolons musste besorgt werden.

# Die Sprache

Es handelt sich um eine leicht modifizierte Version der WHILE-Sprache von Uwe
Schöning. Die Grammatik befindet sich in `grammar.txt` und hier:

```ebnf
Statement -> Sequence

Sequence -> Single | Sequence ; Single

Single ->
    Id = Id + Num |
    Id = IN |
    WHILE Id { Statement } |
    IF Id { Statement } ELSE { Statement } |
    IF Id { Statement } |
    OUT Id

Id -> /[a-z\_][a-z0-9\_]*/
Num -> /\-?[0-9]+/
```

## Modifikationen

- Hinzugefügt
    - `IF Id { ... }`, `IF Id { ... } ELSE { ... }`
    - `Id = IN`, um eine Ganzzahl vom Nutzer zu erfragen und in `Id` zu
        speichern
    - `OUT Id`, um den Wert von `Id` in der Standardausgabe anzuzeigen
    - Zeilen- und Blockkommentare im Stil von C
- Vereinfacht
    - `WHILE (Id != 0)` -> `WHILE Id`
    - Beliebige Variablenbezeichnungen aus Kleinbuchstaben,
        Unterstrichen und Zahlen (außer am Anfang)
- Entfernt
    - `LOOP`-Schleifen
