pub mod caesar;
pub mod rot47;
pub mod morse;
pub mod atbash;
pub mod affine;
pub mod rsa;
pub mod vigenere;

// General Crytographic functions used in conjunction with the modules

//TODO upgrade this to use egcd(a,b) and to use BigInt
// Find the multiplicative inverse by taking in a number
pub fn multiInv(num: isize, modulus: isize) -> u8 {
    for x in 0..(modulus+1) {
        if((num*x) % modulus == 1) {
            return x as u8
        }
    }
    println!("Mod Inv doesn't exist {} {}", num, modulus);
    0
}

//TODO make all rot 1 master class with custom alphabet