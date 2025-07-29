use mydev_rust;

fn main() {

    let  a = mydev_rust::string_to_float ( );
    let  b = mydev_rust::string_to_float ( );


        println!("La suma de {:.2} y {:.2} es : {:.2}", a , b, suma(a, b));

}

fn suma(n1 : f64, n2 : f64) -> f64 {
    n1 + n2
}
