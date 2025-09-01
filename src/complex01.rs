/*
Add the num crate to your Cargo.toml file: 
[dependencies]
num = "0.4" # Use the latest version of num

To work with complex numbers in Rust, use the num crate,
 specifically num::complex::Complex, which provides a generic Complex struct with re (real) 
 and im (imaginary) public fields. You create a new complex number using 
 Complex::new(real_part, imaginary_part) and can perform standard operations 
 like addition and multiplication, or access the real 
 and imaginary components directly using the . operator. 

*/
// Import the Complex struct from the num::complex module
use num::complex::Complex;

fn main() {
    // Create a complex number from its real and imaginary parts
    let z1 = Complex::new(3.0, 4.0); // 3 + 4i
    let z2 = Complex::new(-1.0, 2.0); // -1 + 2i

    // Perform addition
    let sum = z1 + z2;
    println!("z1: {}, z2: {}", z1, z2); // Output: z1: 3+4i, z2: -1+2i
    println!("Sum: {}", sum);            // Output: Sum: 2+6i

    // Perform multiplication
    let product = z1 * z2;
    println!("Product: {}", product);    // Output: Product: -11+2i

    // Access the real and imaginary parts
    println!("Real part of z1: {}", z1.re); // Output: Real part of z1: 3
    println!("Imaginary part of z1: {}", z1.im); // Output: Imaginary part of z1: 4

    // Other operations like conjugation are also available
    println!("Conjugate of z1: {}", z1.conjugate()); // Output: Conjugate of z1: 3-4i
}