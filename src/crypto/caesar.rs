pub fn decrypt(cipher_text: &str) -> Vec<String> {
    
    let mut plain_text: Vec<String> = Vec::new(); 
    for shift in 0..26 {
        let mut temp = String::new();
        for c in cipher_text.chars() {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    temp.push(((c as u8 - 97 + shift)%26 + 97) as char);
                } else {
                    temp.push(((c as u8 - 65 + shift)%26 + 65) as char);
                }
            } else {
                temp.push(c);
            }
        }
        plain_text.push(temp);
    }
    plain_text
}
