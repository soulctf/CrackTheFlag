mod crypto {
    
   

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
    //println!("{:?}", crypto::morse("..../..-"));
}