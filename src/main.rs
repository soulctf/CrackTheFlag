mod crypto;

fn main() {
    use crypto::rot47;
    println!("{:?}", rot47::rot47("A:4@r%uL4p6D2#0r:!9tC$0;F$%02#t?%0Dt4&CtN"));
}