<<<<<<< HEAD
mod crypto;

fn main() {
    use crypto::rot47;
    println!("{:?}", rot47::rot47("A:4@r%uL4p6D2#0r:!9tC$0;F$%02#t?%0Dt4&CtN"));
=======
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

    pub fn morse(data: &str) {
        use std::collections::HashMap;

        // Takes in data and converts it to a String
        let data = data.to_string();
        // Splits string by a whitespace character
        let msg = data.split(" ");
        // Creates a preinitilaized hash table in the form of an array for morse code; basically a large static lookup table
        let md: HashMap<String, char> = [
            // Standard Morse Dictionary(key, value)
            (".-".to_string(), 'A'),
            ("-...".to_string(), 'B'),
            ("-.-.".to_string(), 'C'),
            ("-..".to_string(), 'D'),
            (".".to_string(), 'E'),
            ("..-.".to_string(), 'F'),
            ("--.".to_string(), 'G'),
            ("....".to_string(), 'H'),
            ("..".to_string(), 'I'),
            (".---".to_string(), 'J'),
            ("-.-".to_string(), 'K'),
            (".-..".to_string(), 'L'),
            ("--".to_string(), 'M'),
            ("-.".to_string(), 'N'),
            ("---".to_string(), 'O'),
            (".--.".to_string(), 'P'),
            ("--.-".to_string(), 'Q'),
            (".-.".to_string(), 'R'),
            ("...".to_string(), 'S'),
            ("-".to_string(), 'T'),
            ("..-".to_string(), 'U'),
            ("...-".to_string(), 'V'),
            (".--".to_string(), 'W'),
            ("-..-".to_string(), 'X'),
            ("-.--".to_string(), 'Y'),
            ("--..".to_string(), 'Z'),
            ("-----".to_string(), '0'),
            (".----".to_string(), '1'),
            ("..---".to_string(), '2'),
            ("...--".to_string(), '3'),
            ("....-".to_string(), '4'),
            (".....".to_string(), '5'),
            ("-....".to_string(), '6'),
            ("--...".to_string(), '7'),
            ("---..".to_string(), '8'),
            ("----.".to_string(), '9'),
            (".-...".to_string(), '&'),
            (".--.-.".to_string(), '@'),
            ("-.--.-".to_string(), ')'), 
            ("-.--.".to_string(), '('),
            ("---...".to_string(), ':'),
            ("--..--".to_string(), ','), 
            ("-...-".to_string(), '='),
            ("-.-.--".to_string(), '!'),
            (".-.-.-".to_string(), '.'),
            ("-....-".to_string(), '-'), 
            (".-.-.".to_string(), '+'),
            ("..--..".to_string(), '?'),
            ("-..-.".to_string(), '/')
        // Iterates, clones, and collects all values from the map so that they can be looped though
        ].iter().cloned().collect();
        // Creates and Empty String to store concatenated characters
        let mut unencrypted = String::new();
        // Loops through both the message and the map and compares each instance of morse to values within the map
        for instance in msg {
            for (key, value) in &md {
                if instance == key {
                    // Dereferences the value and pushes it onto a string
                    unencrypted.push(*value);
                }
            }
        }
        // Prints out the final unencrypted string
        println!("{}", unencrypted);
    }

    
}

fn main() {
    println!("{:?}", crypto::caesar("ebiil tloia", 3));
    println!("{:?}", crypto::morse(".--- .... -.-. - ..-. .---- ...-- ...-- --... -- ----- .-. ..... ...--"));
>>>>>>> 050895cd7a6286807387f0960034efc0f8ac35bd
}