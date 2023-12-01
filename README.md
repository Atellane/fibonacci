# Projet Fibonacci
Ce projet est un petit exercice réalisé afin de mettre en pratique mes connaissances fraichement acquises en **Rust**.

Pour ce projet, je vais demander une valeur dont mon programme se servira pour calculer le terme correspondant de la suite 
de Fibonacci, je dois donc commencer par importer la crate *input/output* de la *bibliothèque standard*, `std::io` :
```Rust
use std::io::stdin;

fn main() {
    // --code coupé--
}
```
`std` correspond à la *biliothèque standard*, `io` à la crate *input/output* et `stdin` à l'entrée standard permettant de demander des valeurs.
## récupération du terme à calculer
Commençons par demander une valeur correspondant aux terme de la suite de fibonnacci à calculer :
```Rust
use std::io::stdin;

fn main() {
    println!("entrer le n° du terme de la suite de fibonacci que vous voulez calculer :");

    let mut numero_du_terme_de_la_suite_a_calculer: String = String::new();

    stdin()
        .read_line(&mut numero_du_terme_de_la_suite_a_calculer)
        .expect("Le programme n'as pas pu lire l'entrée !");
    
    // --code coupé--
}
```
Le code ci-dessus affiche d'abord un message avec `println!` pour prévenir l'utilisateur qu'une valeur lui ai demandée. Ensuite on crée une variable muable 
`numero_du_terme_de_la_suite_a_calculer` de type `String` à laquelle nous assignons pour valeur une nouvelle chaine de caractère. La fonction qu'on utilise pour
demander la valeur à l'utilistaeur est `stdin()` (= standard input), `.read_line` lit la valeur qui a été entrée et la place dans la variable
`numero_du_terme_de_la_suite_a_calculer`. Enfin `.expect` se charge d'afficher un message d'erreur si `read_line` n'a pas fonctionné.

Maintenant que nous savons quelle valeur notre utilisateur veut rentre, il faut convertir cette valeur `String` en `u64` (non signé car pas de terme négatif pour une
suite) pour pouvoir être utilisée en paramètre de notre fonction.
```Rust
use std::io;
fn main() {
    {
        // --code coupé--
        
        let mut numero_du_terme_de_la_suite_a_calculer: u64 = numero_du_terme_de_la_suite_a_calculer.trim().parse::<u64>().unwrap();

        // --code coupé--
    }
    
    // --code coupé--
}
```
Tout d'abord, on utilise l'ombrage de **Rust** pour créer une nouvelle variable `temperature_en_celsius` de type `u64` qui [^1]*ombragera* la première, sa valeur 
est le résultat de la conversion de l'ancienne variable du même nom, en `u64`. `.trim()` s'occupe de retirer les espaces au début et à la fin de la chaine de 
caractère d'origine, ensuite `.parse::<u64>()` convertit la chaine de caractère en `u64` et `.unwrap` affiche un message d'erreur si `parse` n'as pas fonctionné. 
## fonctions fibonnaci
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
Le 2er termes étant égaux à 1, on assigne le 1er terme à `i` car comme ça cela exécute 0+1 pour le 2e terme et non 1+1. si `i` est égal à `nombre_de_terme_a_calculer` 
c'est qu'on à atteint le terme à calculer et qu'on peut l'afficher.
[^1]: une variable qui ombrage une variable porte le même nom qu'une ancienne variable et la remplacera dans la portée interne