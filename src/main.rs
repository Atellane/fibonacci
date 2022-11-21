use std::io::stdin;

fn main() {
    println!("entrer le n° du terme de la suite de fibonacci que vous voulez calculer :");

    let mut numero_du_terme_de_la_suite_a_calculer: String = String::new();
    
    stdin()
        .read_line(&mut numero_du_terme_de_la_suite_a_calculer)
        .expect("le programme n'a pas pu lire l'entrée");
    
    let numero_du_terme_de_la_suite_a_calculer: i32 = numero_du_terme_de_la_suite_a_calculer.trim().parse::<i32>().unwrap() + 1;

    suite_de_fibonacci(numero_du_terme_de_la_suite_a_calculer);
}

fn suite_de_fibonacci(nombre_de_terme_a_calculer: i32) -> () {
    let mut nombre0: i32 = 0;
    let mut nombre1: i32 = 1;
    let mut suivant: i32;
    let mut i: i32 = 0;
    let indice_instance_de_la_suite: i32 = nombre_de_terme_a_calculer - 1;
    
    while i < nombre_de_terme_a_calculer {
        if i <= 1 {
            suivant = i;
        }
        else {
            suivant = nombre0 + nombre1;
            nombre0 = nombre1;
            nombre1 = suivant;
        }
        i = i + 1;
        if i == nombre_de_terme_a_calculer {
            println!("le nombre n°{nombre_de_terme_a_calculer} f({indice_instance_de_la_suite}) est {suivant} !");
        }
    }
}
