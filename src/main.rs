mod crypto;

fn main() {
    use crypto::*;
    println!("{:?}", "Compiled");
    println!("{:?}", crypto::vigenere::encrypt("SUPERSECRET", "code"));
        println!("{:?}", crypto::vigenere::decrypt("UISITGHGTSW", "code"));

}