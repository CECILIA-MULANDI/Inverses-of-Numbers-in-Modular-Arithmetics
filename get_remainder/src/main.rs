fn main() {
    let a=7;
    let modulo=5;
    println!("The remainder of {} mod {} is: {:?}",a,modulo,get_remainder(a,modulo));
    let b=36;
    let power1=4;
    let c=1;
    let power2=1;
    let modulo=5;
    println!("The remainder of {}^{} * {}^{} mod {} is:{:?}",b,power1,c,power2,modulo,get_remainder_of_numbers_with_exponents(b,power1,c,power2,modulo));
}
// when we have a mod m
// a is usually the remainder
// so if we did 14mod5
// remainder will be 4
// in cases where the remainder is negative we make it positive
// for example:
// -14 mod 5  = -4 but we make it positive by adding 5 to get 1
fn get_remainder(a:i128,modulo:i128)->u128{
    let remainder=(a%modulo+modulo)%modulo;
    remainder.try_into().unwrap()
}
// if we have (a^n*b^t) mod m
// (2^3 * 4^2) mod 9
fn get_remainder_of_numbers_with_exponents(a:i128,power1:u32,b:i128,power2:u32,modulo:i128)->i128{
    let remainder=(a.pow(power1)*b.pow(power2))%modulo;
    remainder.try_into().unwrap()
}