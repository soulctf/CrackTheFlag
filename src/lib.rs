pub mod crypto;

#[cfg(test)]
mod tests {

    use super::crypto::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn crypto_test(){
        // Tests to see if we get the desired output
        assert_eq!(caesar::caesar_cipher("Test123")[19], "Mxlm123");
        assert_eq!(rot47::rot47("A:4@r%uL4p6D2#0r:!9tC$0;F$%02#t?%0Dt4&CtN")[14], "picoCTF{cAesaR_CiPhErS_juST_aREnT_sEcUrE}");
        assert_eq!(morse::morse("- . ... - .---- --... ---.. ....- -.... ...-- ..--.. .-..."), "TEST178463?&");
        assert_eq!(atbash::atbash("zggzxp zg wzdm"), "attackatdawn");
    }
}
