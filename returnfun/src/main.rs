fn main() {
    println!("suma es {}",sum(1,2));
    println!("suma es {}",sum2(1,4));
}

// first form implicit
fn sum(a:i32,b:i32) -> i32{
    a + b
}

//second form explicit
fn sum2(a:i32,b:i32) -> i32{
    return a + b;
}

