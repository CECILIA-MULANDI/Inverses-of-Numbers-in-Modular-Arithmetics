fn main() {
    let res=find_multiplicative_inverse(2,8);
    match res{
        Some(res)=>println!("{:?}",res),
        None=>println!("The is no inverse for this solution")
    }
   
}
fn extended_euclidean_algorithm(a:i128,b:i128)->(i128,i128,i128){
    if b==0{
        return (a,1,0)
    }
    let (gcd,x1,y1)=extended_euclidean_algorithm(b,a%b);
    let x=y1;
    let y=x1-(a/b)*y1;
    (gcd,x,y)
}

fn find_multiplicative_inverse(a:i128,modulo:i128)->Option<i128>{
    let (gcd,x,_)=extended_euclidean_algorithm(a,modulo);
    if gcd !=1{
        None
    }
    else{
        Some((x%modulo+modulo)%modulo)
    }
   
}
