pub mod caesar;
pub mod rot47;
pub mod morse;
pub mod atbash;
pub mod affine;
pub mod rsa;
pub mod vigenere;

//TODO upgrade this to use egcd(a,b) and to use BigInt
pub fn modInv(num: isize, module: isize) -> u8 {
    for x in 0..(module+1) {
        if((num*x) % module == 1) {
            return x as u8
        }
    }
    println!("Mod Inv doesn't exist {} {}", num, module);
    0
}

//TODO make all rot 1 master class with custom alphabet