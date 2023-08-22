fn main() {
    println!("Hello, world!");

    main1();
    main2(3)
}

fn main1(){
    println!("main1 function");
}

fn main2(number: i32){
    println!("{}", number)
}