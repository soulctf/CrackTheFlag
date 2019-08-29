extern crate num;

use num::bigint::BigInt;

struct rsa {
    n: BigInt,
    e: BigInt,
    c: BigInt, 
    d: BigInt,
    p: BigInt, 
    q: BigInt, 
    phi: BigInt,
}

