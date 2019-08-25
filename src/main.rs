mod crypto {
    
    use std::char;

    // All crypto functions will go here
    pub fn caesar(text: &str, shift: u8) -> String {
        
        let mut encrypted_text = String::new(); 
        for c in text.chars() {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    encrypted_text.push(((c as u8 - 97 + shift)%26 + 97) as char)
                } else {
                    encrypted_text.push(((c as u8 - 65 + shift)%26 + 65) as char)
                }
            } else {
                encrypted_text.push(c)
            }
            
        }
        encrypted_text
    }

    fn rot47() {

    }

    pub fn morse(data: &str) {
        let encrypted = data.to_string();
        let mut v = Vec::new();
        for i in encrypted.chars() {
            // println!("{:?}", i)
            v.push(i);
        }
        println!("{:?}", v);

    }

    
}

fn main() {
    println!("{:?}", crypto::caesar("ebiil tloia", 3));
    println!("{:?}", crypto::morse("..../..-"));
}