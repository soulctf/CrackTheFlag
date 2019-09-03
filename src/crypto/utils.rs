use num_bigint::*;
use num_traits::*;
use num::integer::gcd;
// General Crytographic functions used in conjunction with the modules

//TODO upgrade this to use egcd(a,b) and to use BigInt
// Find the multiplicative inverse by taking in a number
pub fn multiInv(num: isize, modulus: isize) -> u8 {
    for x in 0..(modulus+1) {
        if (num*x) % modulus == 1 {
            return x as u8
        }
    }
    println!("Mod Inv doesn't exist {} {}", num, modulus);
    0
}
fn egcd(a: &BigInt, b: &BigInt) {
    // Formula: ax + by = gcd(a, b) -> Solving for x & y
    // Clones both values that are used in determining gcd(quoti)
    let mut a = a.clone();
    let mut b = b.clone();

}
//TODO make all rot 1 master class with custom alphabet