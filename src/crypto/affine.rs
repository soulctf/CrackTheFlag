//TODO: affine Cipher
use crate::crypto::multiInv;

// Takes in a string and returns a vector
// Formulas:
// Encryption -> c = ap + b (mod m), 1 <= a <= m, 1 <= b <= m
// Decryption -> p = a^-1(c - b) (mod m)
// Vars:
// - c = ciphertext letter(converted to an int/u8)
// - p = plaintext letter(converted to an int/u8)
// - a = first key; must be between 1 & 26(number of characters in your alphabet)
//       Note: Should have no factors in common with m(26)
// - b = second key; must be between 1 & 26
// - m = total number of characters in the alphabet
pub fn affine(data: &str) -> Vec<String> {
    // Takes in data and converts the reference to an actual string
    let data = String::from_str(data);
    for c in data.chars() {
        // Creates a vector and pushes all character values onto the stack & returns the vector
        let v: Vec<char> = Vec::new();
        v.push(c);
        return v
    }
    

    fn encryption(a: u8, b: u8, p: u8, m: u8 = 26) {

    }

    fn decryption(a: u8, b: u8, c: u8, m: u8 = 26) {

    }
    
    
    }

