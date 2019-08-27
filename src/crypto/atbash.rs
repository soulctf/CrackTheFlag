//TODO: Atbash Cipher
    pub fn atbash(data: &str) -> String {
        let plain_text: String = data.chars().filter_map(codex).collect();

        // Creates a codex which takes an iterable char and returns Some<char> or None
        fn codex(c: char) -> Option<char> {
            match c {
                // matches using an inclusive range of characters then takes the byte value of a,z, and Z and adds them together
                // while subtracting them from c, which is the iterated character from data
                '0'..='9' => Some(c),
                'a'..='z' => Some((b'a' + b'z' - c as u8) as char),
                'A'..='Z' => Some((b'a' + b'Z' - c as u8) as char),
                _ => None

            }
        }
        plain_text

    }