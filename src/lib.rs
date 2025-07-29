#![allow(unused)]
use std::io;




pub fn string_to_float ( ) -> f64 {

    println!("Introduzca un numero :");

        let mut n1 = String::new();

        io::stdin().read_line(&mut n1).expect("cant read line");

        n1.trim().parse::<f64>().expect("no number")
}
