fn main() {
    let a=-7;
    let modulo=19;
    println!("The additive inverse of {a}mod{modulo}={:?}",additive_inverse(a,modulo));
}

fn additive_inverse(a:i128,modulo:i128)->i128{
    (modulo-a%modulo)%modulo
}
