extern crate num;

use num::bigint::BigInt;

pub struct rsa {
    pub n: Option<BigInt>,
    pub e: Option<BigInt>,
    pub c: Option<BigInt>, 
    pub d: Option<BigInt>,
    pub p: Option<BigInt>, 
    pub q: Option<BigInt>, 
    pub phi: Option<BigInt>,
}

impl rsa {

    //Constructor - sets all values to None
    pub fn new() -> rsa {
        rsa { n: None, e: None, c: None, d: None, p: None, q: None, phi: None}
    }

    pub fn decrypt(self) -> String {
        let d = self.d.unwrap();
        let n = self.n.unwrap();
        //Raise c to d, mod n, then grab the bytes and convert to String. Strip properly to only get the resulting String
        String::from_utf8(self.c.unwrap().modpow(&d, &n).to_bytes_be().1).unwrap()
    }
}