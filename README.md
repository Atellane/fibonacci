# Projet Fibonacci
Ce projet est un petit exercice réalisé afin de mettre en pratique mes connaissances fraichement acquises en **Rust**.

Pour ce projet, je vais demander une valeur dont mon programme se servira pour calculer le terme correspondant de la suite 
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
n de la façon suivante :
```math
f_{n} = f_{n-1} + f_{n-2}
```


```math
On vas donc créer une fonction tel que :
f_{n-2} initialisé à 1 au début pour correspondre à f_{0} est représenté par la varible "nombre0", 
f_{n-1} initialisé à 1 au début pour correspondre à f_{1} est représenté par la variable "nombre1" et
f_{n} est représenté par la variable `suivant`.
```
Soit `i` la variable représentant le numéro du terme qu'on calcule actuellement et soit `nombre_de_terme_a_calculer` :
```Rust
fn suite_de_fibonacci(nombre_de_terme_a_calculer: u64) -> () {
    let mut nombre0: u64 = 0;
    let mut nombre1: u64 = 1;
    let mut suivant: u64;
    let mut i: u64 = 1;
    let indice_instance_de_la_suite: u64 = nombre_de_terme_a_calculer - 1;
    
    while i < nombre_de_terme_a_calculer {
        if i <= 1 {
            suivant = i;
        }
        else {
            suivant = nombre0 + nombre1;
            nombre0 = nombre1;
            nombre1 = suivant;
        }
        i += 1;
        if i == nombre_de_terme_a_calculer {
            println!("le nombre n°{nombre_de_terme_a_calculer} f({indice_instance_de_la_suite}) est {suivant} !");
        }
    }
}
```