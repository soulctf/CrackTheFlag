#[cfg(test)]
mod tests {
    use crate::crypto;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn crypto_test(){
        // Tests to see if we get the desired output
        assert_eq!(crypto::caesar("Test123")[19], "Mxlm123");
        assert_eq!(crypto::rot47("4-'3evh?'c)7%t#e-r,g6u#.9uv#%tg2v#7g'w6gA")[46], "picoCTF{cAesaR_CiPhErS_juST_aREnT_sEcUrE}");
    }
}

mod crypto {
    
    // All crypto functions will go here

    //ROT FUNCTIONS - SHOULD ABSTRACT FOR CUSTOM ALPHABET
    pub fn caesar(cipher_text: &str) -> Vec<String> {
        
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

    pub fn rot47(cipher_text: &str) -> Vec<String> {

        let mut plain_text: Vec<String> = Vec::new();
        for offset in 0..93 {
            let mut temp = String::new();
            for c in cipher_text.chars() {
                if c as u8 >= 33 && c as u8 <= 126 {
                    temp.push((33 + ((c as u8 + offset) % 94)) as char)
                } else {
                    temp.push(c)
                }
            }
            plain_text.push(temp);    
        }
        plain_text
    }

    fn morse(){


    }
    
}