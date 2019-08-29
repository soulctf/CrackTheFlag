pub fn encrypt(text: &str, key: &str) -> String { 
 
    let mut decrypted_text = String::new();
    let key = key.to_ascii_lowercase();
    let mut key_index = 0;
    for c in text.chars() {
        if c.is_alphabetic() {
                if c.is_lowercase() {
                    decrypted_text.push(((((c as i8 + (key.chars().nth(key_index % key.len()).unwrap() as i8 -97) -97))%26 + 26)%26 + 97) as u8 as char);
                    key_index += 1;
                } else {
                    decrypted_text.push(((((c as i8 + (key.chars().nth(key_index % key.len()).unwrap() as i8 -97) -65))%26 + 26)%26 + 65) as u8 as char);
                    key_index += 1;
                }
            } else {
                decrypted_text.push(c);
            }
    }
    decrypted_text
}

pub fn decrypt(text: &str, key: &str) -> String { 
 
    let mut decrypted_text = String::new();
    let key = key.to_ascii_lowercase();
    let mut key_index = 0;
    for c in text.chars() {
        if c.is_alphabetic() {
                if c.is_lowercase() {
                    decrypted_text.push((((c as i8 - (key.chars().nth(key_index % key.len()).unwrap() as i8 -97) -97)%26 + 26)%26 + 97) as u8 as char);
                    key_index += 1;
                } else {
                    decrypted_text.push((((c as i8 - (key.chars().nth(key_index % key.len()).unwrap() as i8 -97) -65)%26 + 26)%26 + 65) as u8 as char);
                    key_index += 1;
                }
            } else {
                decrypted_text.push(c);
            }
    }
    decrypted_text
}