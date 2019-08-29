//TODO: affine Cipher
use crate::crypto::modInv;

pub fn affine(data: &str) -> Vec<String> {
    let mut decrypted_text: Vec<String> = Vec::new();
    for i in 0..26 {
        let inv = modInv(i, 26);
        for j in 0..26 {
            let mut temp = String::new();
            for c in data.to_string().chars() {
                if c.is_alphabetic() {
                    if c.is_lowercase() {
                        temp.push((((c as u8 - 97 - j)*inv + 26)%26 + 97) as char);
                    } else {
                        temp.push((((c as u8 - 65 - j)*inv + 26)%26 + 65) as char);
                    }
                } else {
                    temp.push(c);
                }
            }
            decrypted_text.push(temp);
        }
    }
    decrypted_text
}

