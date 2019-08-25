#[cfg(test)]
mod tests {
    use crate::crypto::caesar;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    fn crypto_test(){
        // Tests to see if we get the desired output
        assert_eq!(caesar("Hello", 5), "Mjqqt")
    }
}

mod crypto {
    
    use std::io;

    // All crypto functions will go here
    pub fn caesar(data: &str, shift: u8) -> String {
        
        let mut characters = data.to_string();
        for i in characters.chars() {
            print!("{:?}", i)
        }
        "Test".to_string()
    }

    fn rot47() {

    }

    fn morse(){

    }

    
}