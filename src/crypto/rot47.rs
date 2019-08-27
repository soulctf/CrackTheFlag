//Returns a Vector of Strings with all possible 
//rot 47 shifts
pub fn rot47(cipher_text: &str) -> Vec<String> {

        let mut plain_text: Vec<String> = Vec::new();
        for offset in 0..93 {
            let mut temp = String::new();
            for c in cipher_text.chars() {
                //Math used to ensure it roles back to the start of the alphabet used
                if c as u8 >= 33 && c as u8 <= 126 {
                    temp.push((33 + ((c as u8 + offset) % 94)) as char)
                } else {
                    temp.push(c)
                }
            }
            //Add the finished rotation to the vector
            plain_text.push(temp);    
        }
        plain_text
    }