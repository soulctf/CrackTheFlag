pub mod crypto;

extern crate num;

#[cfg(test)]
mod tests {

    use super::crypto::*;
    use num::bigint::BigInt;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn crypto_test(){
        // Tests to see if we get the desired output
        assert_eq!(caesar::decrypt("Test123")[19], "Mxlm123");
        assert_eq!(rot47::rot47("A:4@r%uL4p6D2#0r:!9tC$0;F$%02#t?%0Dt4&CtN")[14], "picoCTF{cAesaR_CiPhErS_juST_aREnT_sEcUrE}");
        assert_eq!(morse::morse("- . ... - .---- --... ---.. ....- -.... ...-- ..--.. .-..."), "TEST178463?&");
        assert_eq!(atbash::atbash("zggzxp zg wzdm"), "attackatdawn");

        let mut rsa = rsa::rsa::new();
        rsa.n = BigInt::parse_bytes(b"30994968412821274638126108542140224647370292100079091608343041083209715023181825537637957453183815788151099869840363450721", 10);
        rsa.e = BigInt::parse_bytes(b"65537", 10);
        rsa.c = BigInt::parse_bytes(b"3740808283126743789473658216888004237756151970385422112230702175214670415045578511813428786937523016996521109011952458274", 10);
        rsa.d = BigInt::parse_bytes(b"10949944362147351445695313961215384000802056441294706923101734114824865877971959648683318864984560110549528540371119079473", 10);
        assert_eq!(rsa::rsa::decrypt(rsa), "My credit card number is 1337");
    }
}
