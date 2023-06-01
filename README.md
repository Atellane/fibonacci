# Projet Fibonacci
Ce projet est un petit exercice réalisé afin de mettre en pratique mes connaissances fraichement acquises en **Rust**.

Pour ce projet, je vais demander une valeur dont  mon programme se servira pour calculer le terme correspondant de la suite 
de Fibonacci, je dois donc commencer par importer la crate *input/output* de la *bibliothèque standard*, `std::io` :
```Rust
use std::io;

fn main() {
    // --code coupé--
}
```
`std` correspond à la *biliothèque standard* et `io` à la crate *input/output*.

## fonctions fibonnaci
### créations de notre fonction suite de Fibonnaci
Pour créer notre fonction revoyons comment elle se calcule. La suite de Fibonnaci se calcule en additionnant les deux termes précédant et est défini en fonction de 
n de la façcon suivante :
```math
$f_{n} = {n-1} + {n-2}$
```

